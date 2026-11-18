// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

use crate::ErrorCode;

// trait for writing on the shift register
pub trait ShiftRegister {
    // write function for 8 bytes
    fn write(&self, value: usize) -> Result<(), ErrorCode>;
}
