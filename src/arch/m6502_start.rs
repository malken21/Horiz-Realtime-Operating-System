/// MOS 6502 ブートエントリーポイント (RESET Vector)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m6502_start() -> ! {
    // 1. スタックポインタ ($S = 0xFF) の初期化
    // 2. 割込み禁止 (SEI)
    // 3. 10進モード解除 (CLD)
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {
        // アイドルループ
    }
}
