/// Motorola 68000 ブートエントリーポイント
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m68k_start() -> ! {
    // 1. スタックポインタ (A7) の初期化
    // 2. VBR (Vector Base Register) の設定
    // 3. マシンステータスレジスタ (SR) の初期化
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
