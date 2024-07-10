#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = 32,
    Keyboard = 33,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}