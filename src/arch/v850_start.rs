/// V850 ブートエントリーポイント
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v850_start() -> ! {
    // 1. スタックポインタ (r3) の初期化
    // 2. グローバルポインタ (r4)、テキストポインタ (r5) の初期化
    // 3. 割り込みベクタテーブル (RBASE/INTBP) の設定
    // 4. horiz_rtos_init の呼び出し
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {
        // アイドル
    }
}
