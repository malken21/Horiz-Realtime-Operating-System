// use crate::kernel::scheduler::Scheduler;

/// RISC-V ブートエントリーポイント
/// リセット直後に呼び出される（crt0の一部）
#[unsafe(no_mangle)]
pub unsafe extern "C" fn riscv_start() -> ! {
    // 1. スタックポインタの設定 (sp)
    // 2. グローバルポインタの設定 (gp)
    // 3. トラップハンドラのアドレス設定 (mtvec)
    // 4. BSS領域の初期化
    
    horiz_rtos_init();
}

/// OS初期化シーケンス
fn horiz_rtos_init() -> ! {
    // カーネルコンポーネントの初期化
    // スケジューラの起動
    loop {
        // アイドルループ
    }
}
