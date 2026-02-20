/// Microchip(Atmel) AVR アーキテクチャ用のコンテキスト構造体
/// 8bit マイコン。携帯型医療機器や環境モニタリングなどで広く利用。
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct AvrContext {
    pub r0_to_r31: [u8; 32], // 汎用レジスタ r0-r31
    pub sreg: u8,            // ステータスレジスタ
    pub pc: u16,             // プログラムカウンタ
    pub sp: u16,             // スタックポインタ
}

impl AvrContext {
    pub const fn new() -> Self {
        Self {
            r0_to_r31: [0; 32],
            sreg: 0,
            pc: 0,
            sp: 0,
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
