/// Renesas H8/300H ブートエントリーポイント
#[unsafe(no_mangle)]
pub unsafe extern "C" fn h8_start() -> ! {
    // 1. スタックポインタ (ER7) の初期化
    // 2. 動作モード (アドレッシングモード等) の設定
    // 3. 例外ベクタテーブルのベース設定
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
