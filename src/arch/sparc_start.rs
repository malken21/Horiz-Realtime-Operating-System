/// SPARC V8 ブートエントリーポイント
#[no_mangle]
pub unsafe extern "C" fn sparc_start() -> ! {
    // 1. スタックポインタ (O6) の初期化
    // 2. ウィンドウ無効マスク (WIM) の初期化
    // 3. トラップベースレジスタ (TBR) の設定
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
