/// RISC-V 64bit (RV64) 用のコンテキスト構造体
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Riscv64Context {
    pub x: [u64; 31],   // x1 (ra) to x31 (t6)
    pub pc: u64,        // Program Counter (mepc)
    pub mstatus: u64,   // Machine Status
}

impl Riscv64Context {
    pub const fn new() -> Self {
        Self {
            x: [0; 31],
            pc: 0,
            mstatus: 0,
        }
    }

    pub fn save(&mut self) {
        // RV64 コンテキスト保存
    }

    pub fn restore(&self) -> ! {
        // RV64 コンテキスト復元
        loop {}
    }
}
