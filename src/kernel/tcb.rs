use crate::arch::context::Context;

/// タスク制御ブロック (TCB)
/// タスクの実行状態と管理情報を表す。
pub struct TaskControlBlock {
    pub id: ID,
    pub priority: PRI,
    pub state: TSTAT,
    pub stack_ptr: usize, // スタックポインタ
    pub context: Context,  // 保存されたコンテキスト
}

impl TaskControlBlock {
    pub fn new(id: ID, priority: PRI, stack_ptr: usize) -> Self {
        Self {
            id,
            priority,
            state: TSTAT::Dormant,
            stack_ptr,
            context: Context::new(),
        }
    }
}
