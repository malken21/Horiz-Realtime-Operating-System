/// RISC-V 64bit ブートエントリーポイント
#[no_mangle]
pub unsafe extern "C" fn riscv64_start() -> ! {
    // 1. スタックポインタ (sp) の設定
    // 2. グローバルポインタ (gp) の設定
    // 3. トラップハンドラ (mtvec) の設定
    // 4. BSS領域の初期化
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {
        // アイドルループ
    }
}
