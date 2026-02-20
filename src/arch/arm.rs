/// ARM Cortex-M 用のコンテキスト構造体
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct ArmContext {
    pub r: [u32; 13],   // 汎用レジスタ R0-R12
    pub sp: u32,        // スタックポインタ (R13)
    pub lr: u32,        // リンクレジスタ (R14)
    pub pc: u32,        // プログラムカウンタ (R15)
    pub xpsr: u32,      // プログラムステータスレジスタ
}

impl ArmContext {
    pub const fn new() -> Self {
        Self {
            r: [0; 13],
            sp: 0,
            lr: 0,
            pc: 0,
            xpsr: 0x01000000, // Thumb モードビットをデフォルトでセット
        }
    }

    pub fn save(&mut self) {
        // mrs r0, psp / stmia r1!, {r4-r11} 等の実装
    }

    pub fn restore(&self) -> ! {
        // ldmia r0!, {r4-r11} / msr psp, r0 / bx lr 等による復帰
        loop {}
    }
}
