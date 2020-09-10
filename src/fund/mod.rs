use std::io::{ErrorKind, Result, Error};
use url::Url;
use std::time::SystemTime;
use reqwest;
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json;

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

const SEARCH_API: &'static str = "http://fundsuggest.eastmoney.com/FundSearch/api/FundSearchAPI.ashx";

pub struct App {}

impl<'a> App {
    pub fn new() -> Self {
        App{}
    }

    fn gen_code_detail_url(&self, code: String) -> Url {
        Url::parse(format!("http://fundgz.1234567.com.cn/js/{}.js", code).as_str()).expect("parse detail url error")
    }

    pub fn search(&self, query: &str) -> Result<Vec<Fund>> {
        Err(Error::from(ErrorKind::InvalidData))
    }

    pub async fn get_detail(&self, code: String) -> Result<Fund> {
        let mut url = self.gen_code_detail_url(code);
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        url.query_pairs_mut().append_pair("rt", now.to_string().as_str());

        let text = reqwest::get(url).await.expect("error").text().await.expect("parse error");
        self.to_fund(text).await
    }

    async fn to_fund(&self, text: String) -> Result<Fund> {
        let pattern = match Regex::new(r"jsonpgz\((?P<data>.+)\)") {
            Ok(v) => v,
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.to_string())),
        };
        let res = pattern.captures(&text);

        let json_data = match res {
            Some(v) => v.name("data").expect("err").as_str(),
            _ => return Err(Error::from(ErrorKind::InvalidData)),
        };

        let mut f: Fund = serde_json::from_str(json_data)?;
        f.v_gap += "%";
        Ok(f)
    }
}