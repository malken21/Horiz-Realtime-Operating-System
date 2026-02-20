/// Renesas RL78 アーキテクチャ用のコンテキスト構造体
/// 16bit 超低消費電力マイコン。パチンコ周辺制御や産業用センサで使用。
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Rl78Context {
    pub ax: u16,        // A, X レジスタ (16bitペア)
    pub bc: u16,        // B, C レジスタ
    pub de: u16,        // D, E レジスタ
    pub hl: u16,        // H, L レジスタ
    pub sp: u16,        // スタックポインタ
    pub pc: u32,        // プログラムカウンタ (24bit幅想定)
    pub psw: u8,        // プログラムステータスワード
}

impl Rl78Context {
    pub const fn new() -> Self {
        Self {
            ax: 0, bc: 0, de: 0, hl: 0, sp: 0, pc: 0, psw: 0,
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
