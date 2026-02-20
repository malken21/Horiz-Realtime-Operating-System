/// PowerPC (32-bit) 用のコンテキスト構造体
/// 航空宇宙用 RAD750 等に対応
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct PpcContext {
    pub gpr: [u32; 32], // 汎用レジスタ GPR0-GPR31
    pub msr: u32,       // マシンステータスレジスタ
    pub srr0: u32,      // セーブレジスタ 0 (復帰後の PC)
    pub srr1: u32,      // セーブレジスタ 1 (復帰後の MSR)
    pub lr: u32,        // リンクレジスタ
    pub ctr: u32,       // カウントレジスタ
    pub xer: u32,       // 固定小数点例外レジスタ
    pub cr: u32,        // コンディションレジスタ
}

impl PpcContext {
    pub const fn new() -> Self {
        Self {
            gpr: [0; 32],
            msr: 0,
            srr0: 0,
            srr1: 0,
            lr: 0,
            ctr: 0,
            xer: 0,
            cr: 0,
        }
    }

    pub fn save(&mut self) {
        // stmw r0, 0(r1) 等の保存処理
    }

    pub fn restore(&self) -> ! {
        // rfi による復帰
        loop {}
    }
}
