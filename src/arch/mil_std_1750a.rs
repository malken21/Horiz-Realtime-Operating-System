/// MIL-STD-1750A Architecture Context Implementation
///
/// Ref: MIL-STD-1750A (16-bit computer instruction set architecture)
#[derive(Debug, Default, Clone, Copy)]
pub struct MilStd1750aContext {
    // 16 16-bit General Purpose Registers
    pub r0: u16,  pub r1: u16,  pub r2: u16,  pub r3: u16,
    pub r4: u16,  pub r5: u16,  pub r6: u16,  pub r7: u16,
    pub r8: u16,  pub r9: u16,  pub r10: u16, pub r11: u16,
    pub r12: u16, pub r13: u16, pub r14: u16, pub r15: u16,

    pub ic: u16,  // Instruction Counter
    pub sw: u16,  // Status Word
}

impl MilStd1750aContext {
    pub const fn new() -> Self {
        Self {
            r0: 0, r1: 0, r2: 0, r3: 0,
            r4: 0, r5: 0, r6: 0, r7: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            ic: 0,
            sw: 0,
        }
    }
}
