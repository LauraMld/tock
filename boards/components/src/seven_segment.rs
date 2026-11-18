// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Component for seven-segment display using a shift register.
//! For now, only a seven-segment display is supported
//!
//! SEGMENT_MAP_DIGIT : values for digit select
//! SEVEN_SEGMENT : values for position select
//!
//! Example for nucleo_u545re_q board:
//! ```rust
//! const SEGMENT_MAP_DIGIT: [u8; 10] = [0x03, 0x9F, 0x25, 0x0D, 0x99, 0x49, 0x41, 0x1F, 0x01, 0x09];
//! const SEVEN_SEGMENT: [u8; 4] = [0x8F, 0x4F, 0x2F, 0x1F];
//! ```
//!
//! Usage
//! -----
//! ```rust
//! let seven_segment =
//!     components::seven_segment::SevenSegmentShiftRegisterComponent::new(
//!         sr,
//!         alarm_mux,
//!         SEGMENT_MAP_DIGIT,
//!         SEVEN_SEGMENT,
//!         50,
//!     )
//!     .finalize(components::seven_segment_shift_register_component_static!(
//!         Sr74HC<GpioHw>,
//!         stm32u545::tim::Tim2<'static>,
//!         4
//!     ));
//! ```
//!

use capsules_core::sevensegment::SevenSegmentDriver;
use capsules_core::virtualizers::virtual_alarm::{MuxAlarm, VirtualMuxAlarm};
use capsules_extra::sevensegment::SevenSegmentShiftRegister;
use core::cell::Cell;
use core::mem::MaybeUninit;
use kernel::component::Component;
use kernel::hil::shift_register::ShiftRegister;
use kernel::hil::time::Alarm;

#[macro_export]
macro_rules! seven_segment_shift_register_component_static {
    ($Sr:ty, $A:ty, $DIGITS:expr) => {{
        let alarm = kernel::static_buf!(
            capsules_core::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, $A>
        );

        let display = kernel::static_buf!(
            capsules_extra::sevensegment::SevenSegmentShiftRegister<
                'static,
                $Sr,
                capsules_core::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, $A>,
                $DIGITS,
            >
        );

        let sevensegment = kernel::static_buf!(
            capsules_core::sevensegment::SevenSegmentDriver<
                'static,
                capsules_extra::sevensegment::SevenSegmentShiftRegister<
                    'static,
                    $Sr,
                    capsules_core::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, $A>,
                    $DIGITS,
                >,
            >
        );

        (alarm, display, sevensegment)
    }};
}

pub struct SevenSegmentShiftRegisterComponent<
    Sr: 'static + ShiftRegister,
    A: 'static + Alarm<'static>,
    const DIGITS: usize,
> {
    shift_register: &'static Sr,
    alarm_mux: &'static MuxAlarm<'static, A>,
    segment_map_digit: [u8; 10],
    seven_segment: [u8; DIGITS],
    refresh_rate: usize,
}

impl<Sr: 'static + ShiftRegister, A: 'static + Alarm<'static>, const DIGITS: usize>
    SevenSegmentShiftRegisterComponent<Sr, A, DIGITS>
{
    pub fn new(
        shift_register: &'static Sr,
        alarm_mux: &'static MuxAlarm<'static, A>,
        segment_map_digit: [u8; 10],
        seven_segment: [u8; DIGITS],
        refresh_rate: usize,
    ) -> Self {
        Self {
            shift_register,
            alarm_mux,
            segment_map_digit,
            seven_segment,
            refresh_rate,
        }
    }
}

impl<Sr: 'static + ShiftRegister, A: 'static + Alarm<'static>, const DIGITS: usize> Component
    for SevenSegmentShiftRegisterComponent<Sr, A, DIGITS>
{
    type StaticInput = (
        &'static mut MaybeUninit<VirtualMuxAlarm<'static, A>>,
        &'static mut MaybeUninit<
            SevenSegmentShiftRegister<'static, Sr, VirtualMuxAlarm<'static, A>, DIGITS>,
        >,
        &'static mut MaybeUninit<
            SevenSegmentDriver<
                'static,
                SevenSegmentShiftRegister<'static, Sr, VirtualMuxAlarm<'static, A>, DIGITS>,
            >,
        >,
    );

    type Output = &'static SevenSegmentDriver<
        'static,
        SevenSegmentShiftRegister<'static, Sr, VirtualMuxAlarm<'static, A>, DIGITS>,
    >;

    fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        let alarm = static_buffer.0.write(VirtualMuxAlarm::new(self.alarm_mux));
        alarm.setup();

        let display = static_buffer.1.write(SevenSegmentShiftRegister::new(
            self.shift_register,
            self.segment_map_digit,
            self.seven_segment,
            alarm,
            Cell::new(0),
            Cell::new(0),
            0,
            self.refresh_rate,
            Cell::new(false),
        ));

        alarm.set_alarm_client(display);
        display.init();

        let seven_segment = static_buffer.2.write(SevenSegmentDriver::new(display));

        seven_segment
    }
}
