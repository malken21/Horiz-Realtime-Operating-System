/// x86_64 ブートエントリーポイント (Multiboot2 等から呼び出される)
#[no_mangle]
pub unsafe extern "C" fn x86_64_start() -> ! {
    // 1. GDT (Global Descriptor Table) の初期化
    // 2. IDT (Interrupt Descriptor Table) の初期化
    // 3. ページテーブルの設定
    
    horiz_rtos_init();
}

fn horiz_rtos_init() -> ! {
    loop {}
}
