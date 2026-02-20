use crate::api::types::ID;

/// タスク同期用のセマフォ構造体
pub struct Semaphore {
    pub id: ID,
    pub count: i32,
    pub wait_queue: [Option<ID>; 8], // プロトタイプ用の簡易的な固定サイズキュー
}

impl Semaphore {
    pub fn new(id: ID, initial_count: i32) -> Self {
        Self {
            id,
            count: initial_count,
            wait_queue: [None; 8],
        }
    }

    pub fn wait(&mut self, _task_id: ID) -> bool {
        if self.count > 0 {
            self.count -= 1;
            true
        } else {
            // 実際のカーネルでは、ここでタスクをブロック状態に移行させる
            false
        }
    }

    pub fn signal(&mut self) {
        self.count += 1;
        // 実際のカーネルでは、wait_queueからタスクを起床させる
    }
}
