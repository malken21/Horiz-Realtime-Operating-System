use crate::api::types::{ID, PRI, ER, TSTAT};
use crate::kernel::tcb::TaskControlBlock;

/// タスクの生成 (cre_tsk)
/// 最終的な実装では、TCBとスタックの割り当てを行う。
pub fn cre_tsk(tskid: ID, priority: PRI, stack_ptr: usize) -> ER {
    // シミュレートされた生成ロジック
    if tskid <= 0 {
        return ER::EObj;
    }
    ER::EOk
}

/// タスクの起動 (sta_tsk)
pub fn sta_tsk(tskid: ID) -> ER {
    // タスクをReady状態に移行させるロジック
    ER::EOk
}

/// タスクの終了 (ext_tsk)
pub fn ext_tsk() -> ! {
    // 現在のタスクを終了させるロジック
    loop {}
}
