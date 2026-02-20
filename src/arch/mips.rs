/// MIPS (32bit) 用のコンテキスト構造体
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct MipsContext {
    pub zero: u32,
    pub at: u32,
    pub v0: u32, pub v1: u32,
    pub a0: u32, pub a1: u32, pub a2: u32, pub a3: u32,
    pub t0: u32, pub t1: u32, pub t2: u32, pub t3: u32,
    pub t4: u32, pub t5: u32, pub t6: u32, pub t7: u32,
    pub s0: u32, pub s1: u32, pub s2: u32, pub s3: u32,
    pub s4: u32, pub s5: u32, pub s6: u32, pub s7: u32,
    pub t8: u32, pub t9: u32,
    pub k0: u32, pub k1: u32,
    pub gp: u32,
    pub sp: u32,
    pub fp: u32,
    pub ra: u32,
    pub pc: u32,
    pub status: u32,
    pub cause: u32,
}

impl MipsContext {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn save(&mut self) {
        // MIPS コンテキスト保存
    }

    pub fn restore(&self) -> ! {
        // MIPS コンテキスト復元
        loop {}
    }
}
