#[cfg(target_arch = "riscv32")]
use crate::arch::riscv::RiscvContext as ArchContext;

#[cfg(any(feature = "arch-m68k"))]
use crate::arch::m68k::M68kContext as ArchContext;

#[cfg(any(feature = "arch-z80"))]
use crate::arch::z80::Z80Context as ArchContext;

#[cfg(any(feature = "arch-h8"))]
use crate::arch::h8::H8Context as ArchContext;

#[cfg(any(feature = "arch-arm", all(target_arch = "arm", not(any(feature="arch-m68k", feature="arch-z80", feature="arch-h8", feature="arch-x86_64", feature="arch-powerpc", feature="arch-sparc", feature="arch-v850", feature="arch-rl78", feature="arch-sh", feature="arch-rx", feature="arch-msp430", feature="arch-avr", feature="arch-loongarch64", feature="arch-mips", feature="arch-m6502", feature="arch-arm64", feature="arch-riscv64")))))]
use crate::arch::arm::ArmContext as ArchContext;

#[cfg(any(feature = "arch-x86_64", all(target_arch = "x86_64", not(any(feature="arch-m68k", feature="arch-z80", feature="arch-h8", feature="arch-arm", feature="arch-powerpc", feature="arch-sparc", feature="arch-v850", feature="arch-rl78", feature="arch-sh", feature="arch-rx", feature="arch-msp430", feature="arch-avr", feature="arch-loongarch64", feature="arch-mips", feature="arch-m6502", feature="arch-arm64", feature="arch-riscv64")))))]
use crate::arch::x86_64::X86_64Context as ArchContext;

#[cfg(any(target_arch = "powerpc", feature = "arch-powerpc"))]
use crate::arch::powerpc::PpcContext as ArchContext;

#[cfg(any(target_arch = "sparc", feature = "arch-sparc"))]
use crate::arch::sparc::SparcContext as ArchContext;

#[cfg(any(feature = "arch-v850"))]
use crate::arch::v850::V850Context as ArchContext;

#[cfg(any(feature = "arch-rl78"))]
use crate::arch::rl78::Rl78Context as ArchContext;

#[cfg(any(feature = "arch-sh"))]
use crate::arch::sh::ShContext as ArchContext;

#[cfg(any(feature = "arch-rx"))]
use crate::arch::rx::RxContext as ArchContext;

#[cfg(any(feature = "arch-msp430"))]
use crate::arch::msp430::Msp430Context as ArchContext;

#[cfg(any(feature = "arch-avr"))]
use crate::arch::avr::AvrContext as ArchContext;

#[cfg(not(any(
    target_arch = "riscv32",
    feature = "arch-m68k",
    feature = "arch-z80",
    feature = "arch-h8",
    target_arch = "arm", feature = "arch-arm",
    target_arch = "x86_64", feature = "arch-x86_64",
    target_arch = "powerpc", feature = "arch-powerpc",
    target_arch = "sparc", feature = "arch-sparc",
    feature = "arch-v850",
    feature = "arch-rl78",
    feature = "arch-sh",
    feature = "arch-rx",
    feature = "arch-msp430",
    feature = "arch-avr"
)))]
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultContext {
    pub sp: usize,
    pub pc: usize,
}

#[cfg(not(any(
    target_arch = "riscv32",
    feature = "arch-m68k",
    feature = "arch-z80",
    feature = "arch-h8",
    target_arch = "arm", feature = "arch-arm",
    target_arch = "x86_64", feature = "arch-x86_64",
    target_arch = "powerpc", feature = "arch-powerpc",
    target_arch = "sparc", feature = "arch-sparc",
    feature = "arch-v850",
    feature = "arch-rl78",
    feature = "arch-sh",
    feature = "arch-rx",
    feature = "arch-msp430",
    feature = "arch-avr"
)))]
impl DefaultContext {
    pub const fn new() -> Self {
        Self { sp: 0, pc: 0 }
    }
}

#[cfg(not(any(
    target_arch = "riscv32",
    feature = "arch-m68k",
    feature = "arch-z80",
    feature = "arch-h8",
    target_arch = "arm", feature = "arch-arm",
    target_arch = "x86_64", feature = "arch-x86_64",
    target_arch = "powerpc", feature = "arch-powerpc",
    target_arch = "sparc", feature = "arch-sparc",
    feature = "arch-v850",
    feature = "arch-rl78",
    feature = "arch-sh",
    feature = "arch-rx",
    feature = "arch-msp430",
    feature = "arch-avr"
)))]
use self::DefaultContext as ArchContext;

/// アーキテクチャ非依存のインターフェースとしてのコンテキスト
#[derive(Debug, Default, Clone, Copy)]
pub struct Context(ArchContext);

impl Context {
    pub const fn new() -> Self {
        Self(ArchContext::new())
    }

    pub fn save(&mut self) {
        // self.0.save()
    }

    pub fn restore(&self) -> ! {
        // self.0.restore()
        loop {}
    }
}
