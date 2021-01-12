use std::{error::Error, fmt::Display};

use anyhow::anyhow;
use configparser::ini::Ini;

pub struct Config {
    pub user: String,
    pub password: String,
    pub term_type: String,
}

#[derive(Clone, Debug)]
struct MissingKey(&'static str);

impl Display for MissingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "缺失配置项：gxunet.{}", self.0)
    }
}

impl Error for MissingKey {}

fn from_config(config: &Ini, key: &'static str) -> Result<String, MissingKey> {
    config.get("gxunet", key).ok_or(MissingKey(key))
}

pub fn load() -> anyhow::Result<Config> {
    let mut config = Ini::new();
    config
        .load("config.ini")
        .map_err(|e| anyhow!("读取配置文件失败：{}", e))?;
    let user = from_config(&config, "user")?;
    let password = from_config(&config, "password")?;
    let term_type = from_config(&config, "term_type")?;
    Ok(Config {
        user,
        password,
        term_type,
    })
}
