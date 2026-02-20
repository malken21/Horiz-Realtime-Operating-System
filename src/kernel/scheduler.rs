use crate::kernel::tcb::TaskControlBlock;
use crate::api::types::ID;

pub struct Scheduler {
    // 現時点での簡易実装: プロトタイプ用の固定サイズ配列
    pub tasks: [Option<TaskControlBlock>; 32],
    pub current_task_id: Option<ID>,
}

impl Scheduler {
    pub const fn new() -> Self {
        const EMPTY_TASK: Option<TaskControlBlock> = None;
        Self {
            tasks: [EMPTY_TASK; 32],
            current_task_id: None,
        }
    }

    pub fn schedule(&mut self) -> Option<ID> {
        // 簡易的な優先度ベースの選択 (現時点では数値が大きい方を優先。
        // 今後は、ITRON仕様に合わせて数値が小さいほど高優先度となるよう調整予定)
        let mut best_task: Option<ID> = None;
        let mut highest_priority = -1;

        for task_opt in self.tasks.iter() {
            if let Some(task) = task_opt {
                if task.priority > highest_priority {
                    highest_priority = task.priority;
                    best_task = Some(task.id);
                }
            }
        }

        self.current_task_id = best_task;
        best_task
    }
}
