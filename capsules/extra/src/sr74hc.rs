use kernel::hil::gpio::Configure;
use kernel::hil::{gpio::Output, shift_register::ShiftRegister};

pub struct Sr74HC<P: Configure + Output> {
    latch: P,
    clock: P,
    data: P,
}

impl<'a, P: Configure + Output> Sr74HC<P> {
    pub fn new(latch: P, clock: P, data: P) -> Self {
        latch.make_output();
        clock.make_output();
        data.make_output();

        clock.clear();
        latch.set();
        data.clear();
        Self { latch, clock, data }
    }
}

impl<'a, P: Configure + Output> ShiftRegister for Sr74HC<P> {
    fn write(&self, value: u8) -> Result<(), kernel::ErrorCode> {
        self.latch.clear();
        for i in 0..8 {
            if (value & (1 << i)) != 0 {
                self.data.set();
            } else {
                self.data.clear();
            }

            self.clock.clear();
            self.clock.set();

            self.latch.clear();
            self.latch.set();
        }
        Ok(())
    }
}
