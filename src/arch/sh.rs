/// Renesas SuperH (SH-2) アーキテクチャ用のコンテキスト構造体
/// PLC や産業用制御機器で広く採用されている 32bit RISC
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct ShContext {
    pub r: [u32; 16],   // 汎用レジスタ R0-R15
    pub gbr: u32,       // グローバルベースレジスタ
    pub vbr: u32,       // ベクタベースレジスタ
    pub mach: u32,      // 積和演算レジスタ H
    pub macl: u32,      // 積和演算レジスタ L
    pub pr: u32,        // プロシージャレジスタ (戻り先)
    pub pc: u32,        // プログラムカウンタ
    pub sr: u32,        // ステータスレジスタ
}

impl ShContext {
    pub const fn new() -> Self {
        Self {
            r: [0; 16],
            gbr: 0, vbr: 0, mach: 0, macl: 0, pr: 0, pc: 0, sr: 0,
        }
    }

    pub fn save(&mut self) {
        // STS.L / MOV.L 指令による保存
    }

    pub fn restore(&self) -> ! {
        // LDS.L / MOV.L / RTE による復帰
        loop {}
    }
}
