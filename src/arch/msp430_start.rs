/// TI MSP430 ブートエントリーポイント
#[no_mangle]
pub unsafe extern "C" fn msp430_start() -> ! {
    // 1. ウォッチドッグタイマ (WDT) の停止
    // 2. スタックポインタ (SP) の初期化
    // 3. horiz_rtos_init の呼び出し
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
