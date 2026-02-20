/// Renesas RX アーキテクチャ用のコンテキスト構造体
/// 現代の FA 機器・産業設備の主力となる 32bit CISC
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct RxContext {
    pub r: [u32; 16],   // 汎用レジスタ R0-R15 (R0はSP)
    pub psw: u32,       // プロセッサステータスワード
    pub pc: u32,        // プログラムカウンタ
    pub fpsw: u32,      // 浮動小数点ステータスワード
    pub acc: [u32; 2],  // アキュムレータ
}

impl RxContext {
    pub const fn new() -> Self {
        Self {
            r: [0; 16],
            psw: 0,
            pc: 0,
            fpsw: 0,
            acc: [0; 2],
        }
    }

    pub fn save(&mut self) {
        // PUSHM / STC 指令による保存
    }

    pub fn restore(&self) -> ! {
        // POPM / LDC / RTE による復帰
        loop {}
    }
}
