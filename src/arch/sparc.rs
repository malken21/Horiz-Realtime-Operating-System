/// SPARC V8 (LEON等) 用のコンテキスト構造体
/// 航空宇宙用 LEON2/3/4 プロセッサに対応
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct SparcContext {
    pub g: [u32; 8],    // グローバルレジスタ G0-G7 (G0は常に0)
    pub o: [u32; 8],    // アウトレジスタ O0-O7 (O6はSP)
    pub psr: u32,       // プロセッサステータスレジスタ
    pub wim: u32,       // ウィンドウ無効マスク
    pub tbr: u32,       // トラップベースレジスタ
    pub pc: u32,        // プログラムカウンタ
    pub npc: u32,       // 次のプログラムカウンタ (ディレイスロット用)
    pub y: u32,         // Y レジスタ (乗除算用)
}

impl SparcContext {
    pub const fn new() -> Self {
        Self {
            g: [0; 8],
            o: [0; 8],
            psr: 0,
            wim: 0,
            tbr: 0,
            pc: 0,
            npc: 0,
            y: 0,
        }
    }

    pub fn save(&mut self) {
        // ウィンドウのフラッシュとレジスタ保存
    }

    pub fn restore(&self) -> ! {
        // rett / jmp による復帰
        loop {}
    }
}
