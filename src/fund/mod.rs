use futures::future::join_all;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};
use std::time::SystemTime;
use url::Url;

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

#[derive(Debug, Serialize, Deserialize)]
struct SearchResp {
    #[serde(rename(deserialize = "Datas"))]
    datas: Vec<SearchRespData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchRespData {
    #[serde(rename(deserialize = "CODE"))]
    code: String,

    #[serde(rename(deserialize = "NAME"))]
    name: String,

    #[serde(rename(deserialize = "FundBaseInfo"))]
    base_info: SearchRespBaseInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchRespBaseInfo {
    #[serde(rename(deserialize = "JJJL"))]
    manager: String,
}

pub struct App {}

impl<'a> App {
    pub fn new() -> Self {
        App {}
    }

    // 详情 URL
    fn gen_code_detail_url(&self, code: String) -> Url {
        let mut url = Url::parse(format!("http://fundgz.1234567.com.cn/js/{}.js", code).as_str())
            .expect("parse detail url error");
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        url.query_pairs_mut()
            .append_pair("rt", now.to_string().as_str());
        url
    }

    // 搜索 URL
    fn gen_search_url(&self, keyword: String) -> Url {
        let mut url =
            Url::parse("http://fundsuggest.eastmoney.com/FundSearch/api/FundSearchAPI.ashx")
                .expect("parse search url error");

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        url.query_pairs_mut()
            .append_pair("_", now.to_string().as_str())
            .append_pair("m", "1")
            .append_pair("key", keyword.as_str());
        url
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Fund>> {
        let url = self.gen_search_url(query.to_string());
        let resp = reqwest::get(url)
            .await
            .expect("search error")
            .json::<SearchResp>()
            .await
            .expect("parse error");

        let mut futures = vec![];
        let _: Vec<()> = resp
            .datas
            .iter()
            .map(|v| futures.push(self.get_detail(v.code.to_string())))
            .collect();
        let funds = join_all(futures).await;

        let mut detail_map: HashMap<String, Fund> = HashMap::new();
        let _: Vec<()> = funds
            .into_iter()
            .map(|v| {
                if let Ok(f) = v {
                    detail_map.insert(f.code.to_string(), f);
                };
            })
            .collect();

        let mut res: Vec<Fund> = vec![];
        for data in resp.datas.into_iter() {
            if let Some(detail) = detail_map.get_mut(data.code.as_str()) {
                detail.manager = data.base_info.manager;
                res.push(detail.clone());
            } else {
                res.push(Fund {
                    name: data.name.to_string(),
                    code: data.code.to_string(),
                    manager: data.base_info.manager.to_string(),
                    v_yesterday: "".to_string(),
                    v_today: "".to_string(),
                    v_gap: "".to_string(),
                    v_calc_time: "".to_string(),
                })
            }
        }

        Ok(res)
    }

    pub async fn get_detail(&self, code: String) -> Result<Fund> {
        let url = self.gen_code_detail_url(code);

        let text = reqwest::get(url)
            .await
            .expect("error")
            .text()
            .await
            .expect("parse error");
        self.to_fund(text)
    }

    fn to_fund(&self, text: String) -> Result<Fund> {
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
