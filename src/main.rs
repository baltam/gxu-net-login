use std::time::Duration;

use anyhow::Context;
use config::Config;
use hyper::{client::HttpConnector, Client};

mod config;
mod gxunet;

async fn try_login(client: &mut Client<HttpConnector>, config: &Config) -> anyhow::Result<bool> {
    let params = gxunet::fetch_params(client)
        .await
        .context("请求登录参数失败")?;
    if let Some(params) = params {
        println!("获取到参数：{:?}", params);
        if gxunet::login(client, &params, config)
            .await
            .context("登录失败")?
        {
            println!("登录成功");
            Ok(true)
        } else {
            println!("登录失败，重试");
            Ok(false)
        }
    } else {
        println!("已登录");
        Ok(true)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let config = config::load()?;
    let mut client = Client::new();
    for i in 1..=10 {
        println!("正在尝试登录…… ({}/10)", i);
        match try_login(&mut client, &config).await {
            Ok(true) => return Ok(()),
            Ok(false) => {}
            Err(err) => {
                println!("发生错误：{:?}", err);
            }
        }
        // 休息 2 秒后继续
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    println!("失败次数过多，放弃");
    Ok(())
}
