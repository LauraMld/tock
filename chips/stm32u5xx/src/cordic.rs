// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

use kernel::utilities::StaticRef;

use core::cell::Cell;
use kernel::ErrorCode;
use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::hil::cordic::{Client, Cordic};
use kernel::utilities::cells::OptionalCell;
use tock_registers_v0::interfaces::{Readable, Writeable};
use tock_registers_v0::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers_v0::{register_bitfields, register_structs};

register_structs! {
    pub CordicRegisters {
        /// Control/status register
        (0x00 => pub cr: ReadWrite<u32, CR::Register>),
        /// Argument register
        (0x04 => pub ar: WriteOnly<u32, AR::Register>),
        /// Result register
        (0x08 => pub res: ReadOnly<u32, RES::Register>),
        (0x0C => @END),
    }
}

/// Base address for CORDIC in Secure Alias mode
pub const CORDIC_BASE: StaticRef<CordicRegisters> =
    unsafe { StaticRef::new(0x50021000 as *const CordicRegisters) };

register_bitfields![u32,
    pub CR [
        /// Function selection
        FUNC OFFSET(0) NUMBITS(4) [],

        /// Precision
        PRECISION OFFSET(4) NUMBITS(4) [],

        /// Scaling factor
        SCALE OFFSET(8) NUMBITS(3) [],

        /// Interrupt enable
        IEN OFFSET(16) NUMBITS(1) [],

        /// DMA read enable
        DMAREN OFFSET(17) NUMBITS(1) [],

        /// DMA write enable
        DMAWEN OFFSET(18) NUMBITS(1) [],

        /// Number of results
        NRES OFFSET(19) NUMBITS(1) [],

        /// Number of arguments
        NARGS OFFSET(20) NUMBITS(1) [],

        /// Width of output data
        RESSIZE OFFSET(21) NUMBITS(1) [],

        /// Width of input data
        ARGSIZE OFFSET(22) NUMBITS(1) [],

        /// Result ready flag
        RRDY OFFSET(31) NUMBITS(1) []
    ],

    pub AR [
        /// Input argument
        ARG OFFSET(0) NUMBITS(32) []
    ],

    pub RES [
        /// Output result
        RES OFFSET(0) NUMBITS(32) []
    ]
];

#[derive(Copy, Clone, PartialEq)]
enum State {
    Idle,
    Processing,
}
pub struct CORDIC<'a> {
    registers: StaticRef<CordicRegisters>,
    client: OptionalCell<&'a dyn Client>,
    deferred_call: DeferredCall,
    state: Cell<State>,
}

impl CORDIC<'_> {
    pub fn new(base: StaticRef<CordicRegisters>) -> Self {
        Self {
            registers: base,
            client: OptionalCell::empty(),
            deferred_call: DeferredCall::new(),
            state: Cell::new(State::Idle),
        }
    }
}

impl<'a> Cordic<'a> for CORDIC<'a> {
    fn set_client(&self, client: &'a dyn Client) {
        self.client.set(client);
    }

    fn sin(&self, angle: i32) -> Result<(), ErrorCode> {
        if self.state.get() == State::Processing {
            return Err(ErrorCode::BUSY);
        }

        self.state.set(State::Processing);

        self.registers
            .cr
            .write(CR::FUNC.val(1) + CR::PRECISION.val(6) + CR::NARGS::SET);
        self.registers.ar.set(angle as u32);
        self.registers.ar.set(0x7FFFFFFF);

        self.deferred_call.set();

        Ok(())
    }
}

impl DeferredCallClient for CORDIC<'_> {
    fn handle_deferred_call(&self) {
        if !self.registers.cr.is_set(CR::RRDY) {
            self.deferred_call.set();
            return;
        }

        let result = self.registers.res.get() as i32;
        self.state.set(State::Idle);

        self.client.map(|client| {
            client.sin_done(Ok(result));
        });
    }

    fn register(&'static self) {
        self.deferred_call.register(self);
    }
}
