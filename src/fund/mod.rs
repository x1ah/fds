use std::io::{ErrorKind, Result, Error};
use url::Url;
use std::time::SystemTime;
use reqwest;

#[derive(Debug)]
pub struct Fund<'a> {
    pub name: &'a str,
    pub code: &'a str,
    pub manager: &'a str,
    // 净值日期
    pub v_date: &'a str,
    // 上一个交易日收盘净值
    pub v_yesterday: f64,
    // 今日估算净值
    pub v_today: f64,
    // 今日估算涨跌幅
    pub v_gap: f64,
    // 估算时间
    pub v_calc_time: &'a str,
}

const DETAIL_API: &'static str = "http://fundgz.1234567.com.cn/js/{}.js";
const SEARCH_API: &'static str = "http://fundsuggest.eastmoney.com/FundSearch/api/FundSearchAPI.ashx";

struct App {}

impl App {
    pub fn new() -> Self {
        App{}
    }

    pub fn search(&self, query: &str) -> Result<Vec<Fund>> {
        Err(Error::from(ErrorKind::InvalidData))
    }

    pub fn get_detail(&self, code: &str) -> Result<Fund> {
        let mut url = Url::parse(format!("http://fundgz.1234567.com.cn/js/{}.js", code).as_str()).expect("parse detail url error");
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        url.query_pairs_mut().append_pair("rt", now.to_string().as_str());

        let _ = reqwest::blocking::get(url).expect("get fund detail error");
        Err(Error::from(ErrorKind::NotFound))
    }
}