// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Component for the 74HC595 shift register.
//!
//! Usage
//! -----
//! ```rust
//! let sr = components::shift_register::ShiftRegisterComponent::new(
//!     latch,
//!     clock,
//!     data,
//! )
//! .finalize(components::shift_register_component_static!(
//!     GpioHw
//! ));
//! ```
//!
//!   SEGMENT_MAP_DIGIT[digit]     SEVEN_SEGMENT[position]
//!             |                           |
//!          write()                     write()
//!             |                           |
//!             +-------------+-------------+
//!                           |
//!                           v
//!                 ShiftRegisterComponent
//!                           |
//!                    DATA / CLOCK / LATCH
//!                           |
//!                           v
//!            +---------------------------+
//!            |        74HC595 #1         |
//!            |                           |
//!            |     SEVEN_SEGMENT         |----> Digit selection
//!            |                           |
//!            +-------------+-------------+
//!                         |
//!                     Q7' (SER)
//!                         |
//!                         v
//!            +---------------------------+
//!            |        74HC595 #2         |
//!            |                           |
//!            |    SEGMENT_MAP_DIGIT      |----> Segment selection
//!            |                           |
//!            +---------------------------+
//!                        |
//!                        v
//!                7-Segment Display
//!

use core::mem::MaybeUninit;
use kernel::component::Component;
use kernel::hil::gpio::{Configure, Output};

use capsules_extra::sr74hc::Sr74HC;

#[macro_export]
macro_rules! shift_register_component_static {
    ($P:ty $(,)?) => {{
        let shift_register = kernel::static_buf!(capsules_extra::sr74hc::Sr74HC<$P>);
        shift_register
    }};
}

pub struct ShiftRegisterComponent<P: 'static + Configure + Output> {
    latch: P,
    clock: P,
    data: P,
}

impl<P: 'static + Configure + Output> ShiftRegisterComponent<P> {
    pub fn new(latch: P, clock: P, data: P) -> Self {
        Self { latch, clock, data }
    }
}

impl<P: 'static + Configure + Output> Component for ShiftRegisterComponent<P> {
    type StaticInput = &'static mut MaybeUninit<Sr74HC<P>>;

    type Output = &'static Sr74HC<P>;

    fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        let shift_register = static_buffer.write(Sr74HC::new(self.latch, self.clock, self.data));

        shift_register
    }
}
