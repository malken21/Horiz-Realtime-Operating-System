/// RL78 ブートエントリーポイント
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rl78_start() -> ! {
    // 1. スタックポインタ (SP) の初期化
    // 2. ミラー領域の設定
    // 3. horiz_rtos_init の呼び出し
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
