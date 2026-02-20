/// ARM Cortex-M ブートエントリーポイント
#[unsafe(no_mangle)]
pub unsafe extern "C" fn arm_start() -> ! {
    // 1. スタックポインタ (MSP/PSP) の初期化
    // 2. VTOR (Vector Table Offset Register) の設定
    // 3. FPU の有効化（必要な場合）
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
