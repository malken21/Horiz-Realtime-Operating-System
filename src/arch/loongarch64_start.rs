/// LoongArch 64bit ブートエントリーポイント
#[no_mangle]
pub unsafe extern "C" fn loongarch64_start() -> ! {
    // 1. スタックポインタ ($sp) の設定
    // 2. スレッドポインタ ($tp) の設定
    // 3. 例外ベクタベースアドレス (eentry) の設定
    // 4. BSS領域の初期化
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {
        // アイドルループ
    }
}
