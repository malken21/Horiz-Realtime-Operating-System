/// TRON/ITRON 共通データ型定义
pub type ID = i32;
pub type ATR = u32;
pub type PRI = i32;
pub type STAT = u32;
pub type TMO = i32;

/// エラーコード
pub enum ER {
    EOk = 0,
    ERetry = -1,
    EObj = -63,
}

/// タスク状態
pub enum TSTAT {
    Running = 0x01,
    Ready = 0x02,
    Waiting = 0x04,
    Dormant = 0x08,
}
