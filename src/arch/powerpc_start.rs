/// PowerPC ブートエントリーポイント
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ppc_start() -> ! {
    // 1. スタックポインタ (GPR1) の初期化
    // 2. 例外ベクタテーブル（EVPR/IVPR）の設定
    // 3. データ/命令キャッシュの初期化
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
