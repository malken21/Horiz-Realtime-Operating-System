use crate::api::types::{ID, ATR, ER};

/// イベントフラグ構造体
pub struct EventFlag {
    pub id: ID,
    pub flag: u32,
}

/// イベントフラグの生成 (cre_flg)
pub fn cre_flg(flgid: ID, _attr: ATR, initial_flag: u32) -> ER {
    if flgid <= 0 {
        return ER::EObj;
    }
    ER::EOk
}

/// イベントフラグのセット (set_flg)
pub fn set_flg(flgid: ID, set_ptn: u32) -> ER {
    ER::EOk
}

/// イベントフラグの待ち (wai_flg)
pub fn wai_flg(flgid: ID, wai_ptn: u32, _wait_mode: u32) -> ER {
    ER::EOk
}
