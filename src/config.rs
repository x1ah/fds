use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Error, ErrorKind, Read, Result, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub funds: Vec<String>,
}

impl Config {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let p = match path {
            Some(v) => v,
            None => Config::default_config_path(),
        };

        if !p.exists() {
            return Err(Error::new(ErrorKind::NotFound, "config file not found"));
        }

        let mut w = OpenOptions::new().read(true).write(false).open(p)?;

        let mut buffer = String::new();
        let _ = w.read_to_string(&mut buffer)?;
        let cfg = toml::from_str(buffer.as_str()).unwrap();
        Ok(cfg)
    }

    pub fn default_config_path() -> PathBuf {
        let mut path = match dirs::home_dir() {
            Some(v) => v,
            _ => PathBuf::from("./"),
        };
        path.push(".config/fds");
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }

        path.push("config.toml");
        if path.exists() {
            return path;
        }

        let w = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();

        let mut writer = BufWriter::new(w);
        let _ = writer.write(b"funds = []\n");
        writer.flush().unwrap();
        path
    }
}
