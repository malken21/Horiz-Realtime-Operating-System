/// Zilog Z80 用のコンテキスト構造体
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Z80Context {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub af_prime: u16,
    pub bc_prime: u16,
    pub de_prime: u16,
    pub hl_prime: u16,
    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub pc: u16,
    pub i: u8,
    pub r: u8,
}

impl Z80Context {
    pub const fn new() -> Self {
        Self {
            af: 0, bc: 0, de: 0, hl: 0,
            af_prime: 0, bc_prime: 0, de_prime: 0, hl_prime: 0,
            ix: 0, iy: 0, sp: 0, pc: 0,
            i: 0, r: 0,
        }
    }

    pub fn save(&mut self) {
        // PUSH 命令群による保存相当
    }

    pub fn restore(&self) -> ! {
        // RETI/RETN 等による復帰相当
        loop {}
    }
}
