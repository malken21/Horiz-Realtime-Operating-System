/// SuperH ブートエントリーポイント
#[no_mangle]
pub unsafe extern "C" fn sh_start() -> ! {
    // 1. スタックポインタ (R15) の初期化
    // 2. VBR (ベクタベースレジスタ) の設定
    // 3. horiz_rtos_init の呼び出し
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
