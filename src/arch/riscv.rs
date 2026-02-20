/// RISC-V (RV32) 用のコンテキスト構造体
/// ESP32-C3 等の RISC-V ベースの MCU に対応
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct RiscvContext {
    pub ra: usize,      // Return Address (x1)
    pub sp: usize,      // Stack Pointer (x2)
    pub gp: usize,      // Global Pointer (x3)
    pub tp: usize,      // Thread Pointer (x4)
    pub t0: usize,      // Temporary 0 (x5)
    pub t1: usize,      // Temporary 1 (x6)
    pub t2: usize,      // Temporary 2 (x7)
    pub s0: usize,      // Saved register 0 / Frame pointer (x8)
    pub s1: usize,      // Saved register 1 (x9)
    pub a0: usize,      // Argument 0 (x10)
    pub a1: usize,      // Argument 1 (x11)
    pub a2: usize,      // Argument 2 (x12)
    pub a3: usize,      // Argument 3 (x13)
    pub a4: usize,      // Argument 4 (x14)
    pub a5: usize,      // Argument 5 (x15)
    pub a6: usize,      // Argument 6 (x16)
    pub a7: usize,      // Argument 7 (x17)
    pub s2: usize,      // Saved register 2 (x18)
    pub s3: usize,      // Saved register 3 (x19)
    pub s4: usize,      // Saved register 4 (x20)
    pub s5: usize,      // Saved register 5 (x21)
    pub s6: usize,      // Saved register 6 (x22)
    pub s7: usize,      // Saved register 7 (x23)
    pub s8: usize,      // Saved register 8 (x24)
    pub s9: usize,      // Saved register 9 (x25)
    pub s10: usize,     // Saved register 10 (x26)
    pub s11: usize,     // Saved register 11 (x27)
    pub t3: usize,      // Temporary 3 (x28)
    pub t4: usize,      // Temporary 4 (x29)
    pub t5: usize,      // Temporary 5 (x30)
    pub t6: usize,      // Temporary 6 (x31)
    pub pc: usize,      // Program Counter (mepc)
    pub mstatus: usize, // Machine Status (mstatus)
}

impl RiscvContext {
    pub const fn new() -> Self {
        Self {
            ra: 0, sp: 0, gp: 0, tp: 0, t0: 0, t1: 0, t2: 0, s0: 0,
            s1: 0, a0: 0, a1: 0, a2: 0, a3: 0, a4: 0, a5: 0, a6: 0,
            a7: 0, s2: 0, s3: 0, s4: 0, s5: 0, s6: 0, s7: 0, s8: 0,
            s9: 0, s10: 0, s11: 0, t3: 0, t4: 0, t5: 0, t6: 0,
            pc: 0, mstatus: 0,
        }
    }

    /// コンテキストを保存する
    /// 実際にはインラインアセンブリまたはアセンブリファイルで定義される
    pub fn save(&mut self) {
        // 例: CSR (mepc, mstatus) の取得と汎用レジスタの保存
    }

    /// コンテキストを復元し、実行を再開する
    pub fn restore(&self) -> ! {
        // 例: 汎用レジスタの復元と mret による復帰
        loop {}
    }
}

/// RISC-V 共通トラップハンドラ
#[unsafe(no_mangle)]
pub unsafe extern "C" fn riscv_trap_handler() {
    // 割り込み原因 (mcause) の判定とスケジューラ呼び出し
}
