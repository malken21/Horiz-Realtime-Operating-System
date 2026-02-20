/// x86_64 用のコンテキスト構造体
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct X86_64Context {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64, pub rsp: u64,
    pub r8: u64,  pub r9: u64,  pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cs: u64,
    pub ss: u64,
}

impl X86_64Context {
    pub const fn new() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0,  r9: 0,  r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0,
            rflags: 0x202, // 割り込み有効フラグ
            cs: 0,
            ss: 0,
        }
    }

    pub fn save(&mut self) {
        // pushaq 等に相当する保存処理
    }

    pub fn restore(&self) -> ! {
        // iretq による復帰
        loop {}
    }
}
