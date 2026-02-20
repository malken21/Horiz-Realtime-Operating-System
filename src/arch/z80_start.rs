/// Zilog Z80 ブートエントリーポイント
#[no_mangle]
pub unsafe extern "C" fn z80_start() -> ! {
    // 1. スタックポインタ (SP) の初期化
    // 2. 割込みモード (IM 1 or 2) の設定
    // 3. Iレジスタ (割込みベクトル) の初期化
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
