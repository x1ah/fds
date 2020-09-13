use crate::fund::Fund;
use futures::future::join_all;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};
use std::time::SystemTime;
use url::Url;

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
    base_info: Option<SearchRespBaseInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchRespBaseInfo {
    #[serde(rename(deserialize = "JJJL"))]
    manager: String,
}

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    // 详情 URL
    fn gen_code_detail_url(&self, code: &str) -> Url {
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
    fn gen_search_url(&self, keyword: &str) -> Url {
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
            .append_pair("key", keyword);
        url
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Fund>> {
        let url = self.gen_search_url(query);
        let resp = reqwest::get(url)
            .await
            .expect("search error")
            .json::<SearchResp>()
            .await
            .unwrap();

        let mut futures = vec![];
        let _: Vec<()> = resp
            .datas
            .iter()
            .map(|v| futures.push(self.get_detail(&v.code)))
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
            let manager = match data.base_info {
                Some(v) => v.manager,
                _ => "".to_string(),
            };

            if let Some(detail) = detail_map.get_mut(data.code.as_str()) {
                detail.manager = manager;
                res.push(detail.clone());
            } else {
                res.push(Fund {
                    name: data.name,
                    code: data.code,
                    manager,
                    v_yesterday: "".into(),
                    v_today: "".into(),
                    v_gap: "".into(),
                    v_calc_time: "".into(),
                })
            }
        }

        Ok(res)
    }

    pub async fn get_detail(&self, code: &str) -> Result<Fund> {
        let url = self.gen_code_detail_url(code);

        let text = reqwest::get(url)
            .await
            .expect("error")
            .text()
            .await
            .expect("parse error");
        self.to_fund(&text)
    }

    fn to_fund(&self, text: &str) -> Result<Fund> {
        let pattern = match Regex::new(r"jsonpgz\((?P<data>.+)\)") {
            Ok(v) => v,
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.to_string())),
        };
        let res = pattern.captures(text);

        let json_data = match res {
            Some(v) => v.name("data").expect("err").as_str(),
            _ => return Err(Error::from(ErrorKind::InvalidData)),
        };

        let mut f: Fund = serde_json::from_str(json_data)?;
        f.v_gap += "%";
        Ok(f)
    }
}
