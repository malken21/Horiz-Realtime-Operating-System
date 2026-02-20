/// ARM64 (AArch64) 用のコンテキスト構造体
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Arm64Context {
    pub x: [u64; 31],   // General purpose registers x0-x30
    pub sp: u64,        // Stack Pointer
    pub elr: u64,       // Exception Link Register
    pub spsr: u64,      // Saved Process Status Register
}

impl Arm64Context {
    pub const fn new() -> Self {
        Self {
            x: [0; 31],
            sp: 0,
            elr: 0,
            spsr: 0,
        }
    }

    pub fn save(&mut self) {
        // ARM64 コンテキスト保存
    }

    pub fn restore(&self) -> ! {
        // ARM64 コンテキスト復元
        loop {}
    }
}
