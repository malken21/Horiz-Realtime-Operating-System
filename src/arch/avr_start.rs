/// Microchip(Atmel) AVR ブートエントリーポイント
#[unsafe(no_mangle)]
pub unsafe extern "C" fn avr_start() -> ! {
    // 1. 割り込みベクタテーブル等の初期設定
    // 2. スタックポインタ (SP) の初期化 (SPH, SPL)
    // 3. horiz_rtos_init の呼び出し
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
