/// Motorola 68000 (X68000等) 用のコンテキスト構造体
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct M68kContext {
    pub d: [u32; 8],    // データレジスタ D0-D7
    pub a: [u32; 8],    // アドレスレジスタ A0-A7 (A7はSP)
    pub sr: u16,        // ステータスレジスタ
    pub pc: u32,        // プログラムカウンタ
}

impl M68kContext {
    pub const fn new() -> Self {
        Self {
            d: [0; 8],
            a: [0; 8],
            sr: 0,
            pc: 0,
        }
    }

    pub fn save(&mut self) {
        // movem.l d0-d7/a0-a6, -(sp) 等のアセンブリ相当
    }

    pub fn restore(&self) -> ! {
        // rte による復帰相当
        loop {}
    }
}
