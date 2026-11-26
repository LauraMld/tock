// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Userspace driver for seven-segment displays.
//!
//! ### `command_number`
//!
//! - `0`: Check if the driver is present.
//! - `1`: Return the number of digits available.
//! - `2`: Write a number to the display.
//! - `3`: Clear the display.

use kernel::hil::sevensegment::SevenSegment;

use kernel::ErrorCode;
use kernel::process::{Error, ProcessId};
use kernel::syscall::{CommandReturn, SyscallDriver};

use capsules_core::driver;
pub const DRIVER_NUM: usize = driver::NUM::SevenSegment as usize;

pub struct SevenSegmentDriver<'a, S: SevenSegment> {
    seven_segment: &'a S,
}

impl<'a, S: SevenSegment> SevenSegmentDriver<'a, S> {
    pub fn new(seven_segment: &'a S) -> Self {
        Self { seven_segment }
    }
}

impl<S: SevenSegment> SyscallDriver for SevenSegmentDriver<'_, S> {
    fn command(
        &self,
        command_number: usize,
        r2: usize,
        r3: usize,
        _process_id: ProcessId,
    ) -> CommandReturn {
        match command_number {
            0 => CommandReturn::success(),

            1 => CommandReturn::success_u32(self.seven_segment.num_digits() as u32),

            2 => {
                if r2 > 9999 {
                    CommandReturn::failure(ErrorCode::INVAL)
                } else {
                    let rez = self.seven_segment.write(r2, r3 as u8);
                    if rez == Ok(()) {
                        CommandReturn::success()
                    } else {
                        CommandReturn::failure(ErrorCode::FAIL)
                    }
                }
            }

            3 => {
                let rez = self.seven_segment.clear();
                if rez == Ok(()) {
                    CommandReturn::success()
                } else {
                    CommandReturn::failure(ErrorCode::FAIL)
                }
            }

            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, _process_id: ProcessId) -> Result<(), Error> {
        Ok(())
    }
}
