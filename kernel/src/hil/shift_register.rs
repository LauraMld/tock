use crate::ErrorCode;

pub trait ShiftRegister {
    fn write(&self, value: u8) -> Result<(), ErrorCode>;
}
