/// RX ブートエントリーポイント
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rx_start() -> ! {
    // 1. ユーザスタックポインタ (USP) / 割込スタックポインタ (ISP) の設定
    // 2. INTB (割込テーブルレジスタ) の設定
    // 3. horiz_rtos_init の呼び出し
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
