/// Renesas V850 アーキテクチャ用のコンテキスト構造体
/// パチンコ遊技機の主制御ボードにおけるデファクトスタンダード
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct V850Context {
    pub r: [u32; 32],   // 汎用レジスタ R0-R31 (R0は常に0、R3はGP/SP等)
    pub psw: u32,       // プログラムステータスワード
    pub lp: u32,        // リンクポインタ (R31)
    pub pc: u32,        // プログラムカウンタ (FEPC/EIPC)
}

impl V850Context {
    pub const fn new() -> Self {
        Self {
            r: [0; 32],
            psw: 0,
            lp: 0,
            pc: 0,
        }
    }

    /// コンテキストを保存する
    pub fn save(&mut self) {
        // V850 固有のアセンブリによる保存処理
    }

    /// コンテキストを復元する
    pub fn restore(&self) -> ! {
        // V850 固有のアセンブリによる復帰処理 (RETI等)
        loop {}
    }
}
