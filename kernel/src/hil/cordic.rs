// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

use crate::ErrorCode;

pub trait Client {
    fn sin_done(&self, result: Result<i32, ErrorCode>);
}
pub trait Cordic<'a> {
    fn set_client(&self, client: &'a dyn Client);

    fn sin(&self, angle: i32) -> Result<(), ErrorCode>;
}
