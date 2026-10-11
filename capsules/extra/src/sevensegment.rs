use kernel::ErrorCode;
use kernel::hil::sevensegment::SevenSegment;
use kernel::hil::shift_register::ShiftRegister;

const SEGMENT_MAP_DIGIT: [u8; 10] = [0x03, 0x9F, 0x25, 0x0D, 0x99, 0x49, 0x41, 0x1F, 0x01, 0x09];
const SEVEN_SEGMENT: [u8; 4] = [0x8F, 0x4F, 0x2F, 0x1F];

pub struct SevenSegmentShiftRegister<'a, Sr: ShiftRegister> {
    shift_register: &'a Sr,
    // segment_pins: [u8; 7],
    // point_pin: u8,
    // digit_pins: [u8; 4],
}

impl<'a, Sr: ShiftRegister> SevenSegmentShiftRegister<'a, Sr> {
    pub fn new(
        shift_register: &'a Sr,
        // segment_pins: [u8; 7],
        // point_pin: u8,
        // digit_pins: [u8; 4],
    ) -> Self {
        Self {
            shift_register,
            // segment_pins,
            // point_pin,
            // digit_pins,
        }
    }
}

impl<'a, Sr: ShiftRegister> SevenSegment for SevenSegmentShiftRegister<'a, Sr> {
    fn num_digits(&self) -> u8 {
        4
    }

    fn has_point(&self) -> bool {
        false
    }

    fn write(&self, mut number: u16, point: u8) -> Result<(), ErrorCode> {
        let mut p = 1000;
        for pos in 0..4 {
            let _ = self
                .shift_register
                .write(SEGMENT_MAP_DIGIT[(number / p) as usize]);
            let _ = self.shift_register.write(SEVEN_SEGMENT[pos]);
            number = number % p;
            p = p / 10;
        }

        Ok(())
    }
}
