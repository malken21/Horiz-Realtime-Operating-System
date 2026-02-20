/// MOS 6502 (8bit) 用のコンテキスト構造体
/// 非常に制限されたレジスタセットを持つ
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct M6502Context {
    pub a: u8,      // Accumulator
    pub x: u8,      // Index Register X
    pub y: u8,      // Index Register Y
    pub s: u8,      // Stack Pointer (Offset in page 1)
    pub p: u8,      // Processor Status
    pub pc: u16,    // Program Counter
}

impl M6502Context {
    pub const fn new() -> Self {
        Self {
            a: 0, x: 0, y: 0, s: 0xff, p: 0, pc: 0,
        }
    }

    pub fn save(&mut self) {
        // 6502 コンテキスト保存（通常、スタック経由）
    }

    pub fn restore(&self) -> ! {
        // 6502 コンテキスト復元
        loop {}
    }
}
