// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

use crate::ErrorCode;

//trait for seven segment display
pub trait SevenSegment {
    // returns the number of digits available on the display
    fn num_digits(&self) -> u8;
    fn has_point(&self) -> bool;

    // writes a number on the display
    // returns an ErrorCode
    fn write(&self, number: usize, point: u8) -> Result<(), ErrorCode>;

    // clears the display
    fn clear(&self) -> Result<(), ErrorCode>;
}
