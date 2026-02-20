/// MIPS 32bit ブートエントリーポイント
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mips_start() -> ! {
    // 1. スタックポインタ ($sp) の設定
    // 2. グローバルポインタ ($gp) の設定
    // 3. 例外ハンドラのアドレス設定
    // 4. BSS領域の初期化
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {
        // アイドルループ
    }
}
