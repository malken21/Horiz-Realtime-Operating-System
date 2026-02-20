/// TI MSP430 アーキテクチャ用のコンテキスト構造体
/// 16bit 超低消費電力マイコン。ペースメーカー、血糖値計などポータブル医療機器での採用例が多数。
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Msp430Context {
    pub r4: u16,        // 汎用レジスタ
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub r8: u16,
    pub r9: u16,
    pub r10: u16,
    pub r11: u16,
    pub r12: u16,
    pub r13: u16,
    pub r14: u16,
    pub r15: u16,
    pub pc: u32,        // プログラムカウンタ (MSP430X を考慮して上位拡張幅を持たせる場合は u32)
    pub sr: u16,        // ステータスレジスタ (r2)
    pub sp: u32,        // スタックポインタ (r1)
}

impl Msp430Context {
    pub const fn new() -> Self {
        Self {
            r4: 0, r5: 0, r6: 0, r7: 0, r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0, pc: 0, sr: 0, sp: 0,
        }
    }

    pub fn save(&mut self) {
        // PUSH 指令等を用いたスタックへの保存
    }

    pub fn restore(&self) -> ! {
        // POP 指令等を用いた復帰
        loop {}
    }
}
