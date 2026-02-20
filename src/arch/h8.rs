/// Renesas H8/300H 用のコンテキスト構造体
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct H8Context {
    pub er: [u32; 8],   // 拡張レジスタ ER0-ER7 (ER7はSP)
    pub ccr: u8,        // コンディションコードレジスタ
    pub pc: u32,        // プログラムカウンタ
}

impl H8Context {
    pub const fn new() -> Self {
        Self {
            er: [0; 8],
            ccr: 0,
            pc: 0,
        }
    }

    pub fn save(&mut self) {
        // STM.L ER0-ER7, @-SP 等のプロセッサ固有命令相当
    }

    pub fn restore(&self) -> ! {
        // RTE による復帰相当
        loop {}
    }
}
