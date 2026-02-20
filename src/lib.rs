#![no_std]
#![no_main]

pub mod api;
pub mod kernel;
pub mod arch;

use core::panic::PanicInfo;

/// カーネルパニックハンドラ
/// 産業・航空宇宙環境では、ハードウェアを安全状態（セーフステート）に移行させるか、
/// ウォッチドッグによるリセットをトリガーする必要がある。
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
