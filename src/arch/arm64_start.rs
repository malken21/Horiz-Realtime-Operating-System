/// ARM64 ブートエントリーポイント
#[no_mangle]
pub unsafe extern "C" fn arm64_start() -> ! {
    // 1. スタックポインタ (sp_el1) の設定
    // 2. 例外レベル (EL) の確認と遷移
    // 3. ベクタテーブル (vbar_el1) の設定
    // 4. BSS領域の初期化
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {
        // アイドルループ
    }
}
