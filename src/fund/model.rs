use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fund {
    pub name: String,

    #[serde(rename(deserialize = "fundcode"))]
    pub code: String,

    #[serde(default)]
    pub manager: String,

    // 上一个交易日收盘净值
    #[serde(default, rename(deserialize = "dwjz"))]
    pub v_yesterday: String,

    // 今日估算净值
    #[serde(default, rename(deserialize = "gsz"))]
    pub v_today: String,

    // 今日估算涨跌幅: -1.2 => -1.2%
    #[serde(default, rename(deserialize = "gszzl"))]
    pub v_gap: String,

    // 估算时间
    #[serde(default, rename(deserialize = "gztime"))]
    pub v_calc_time: String,
}

impl Clone for Fund {
    fn clone(&self) -> Fund {
        Fund {
            name: self.name.to_string(),
            code: self.code.to_string(),
            manager: self.manager.to_string(),
            v_yesterday: self.v_yesterday.to_string(),
            v_today: self.v_today.to_string(),
            v_gap: self.v_gap.to_string(),
            v_calc_time: self.v_calc_time.to_string(),
        }
    }
}

impl fmt::Display for Fund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}\t{}\t{}\t{}", self.code, self.name, self.v_gap, self.v_calc_time, self.manager)
    }
}