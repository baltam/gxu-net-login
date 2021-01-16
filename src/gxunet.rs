use std::{collections::HashMap, convert::TryFrom};

use hyper::{client::HttpConnector, Body, Client, Request, Uri};
use itertools::Itertools;
use url::Url;

use crate::config::Config;

static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
        AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.141 \
        Safari/537.36 Edg/87.0.664.75";

#[derive(Debug)]
pub struct GxunetParams {
    user_ip: String,
    ac_name: String,
    ac_ip: String,
    user_mac: String,
}

/// 构造一个 HTTP GET 请求
fn http_get(url: &str) -> Request<Body> {
    Request::get(Uri::try_from(url).unwrap())
        .header("User-Agent", USER_AGENT)
        .body(Body::empty())
        .unwrap()
}

/// 获取登录参数
/// - 若返回 `Ok(Some(params))`，说明需要进行登录
/// - 若返回 `Ok(None)`，说明已经登录
/// - 若返回 `Err(err)`，说明发生了 HTTP 请求错误，需要重试
pub async fn fetch_params(
    client: &mut Client<HttpConnector>,
) -> hyper::Result<Option<GxunetParams>> {
    // 访问连接测试 URL
    let req = http_get("http://t.cn/?isReback=1");
    let resp = client.request(req).await?;
    // 检查 HTTP 状态码是否是 302 Found
    assert_eq!(resp.status(), 302);
    // 获取重定向目标地址
    let location = resp.headers().get("location").unwrap().to_str().unwrap();
    if location.starts_with("http://172.17.0.2/a79.htm?") {
        // 如果是网关 URL，说明需要登录
        // 解析 URL 以获取 query string
        let url = Url::parse(location).unwrap();
        // 将 query_pairs 返回的迭代器转换成 HashMap
        let query = url.query_pairs().collect::<HashMap<_, _>>();

        // 对网关返回的 MAC 地址需要进行映射，截取 48 位十六进制中的前 12 位，并插入分隔符
        let mac_raw = &query["wlanusermac"];
        assert_eq!(mac_raw.len(), 48);
        let mac = (0..6).map(|i| &mac_raw[i * 2..i * 2 + 2]).join("-");

        // 由于 query_pairs 返回的元素类型是 Cow<str>，而 Cow<T> 是 !Copy，
        // 需要先 clone 再 into_owned（这个过程中只发生一次拷贝）
        Ok(Some(GxunetParams {
            user_ip: query["wlanuserip"].clone().into_owned(),
            ac_name: query["wlanacname"].clone().into_owned(),
            ac_ip: query["wlanacip"].clone().into_owned(),
            user_mac: mac,
        }))
    } else if location.starts_with("http://weibo.com/") {
        // 如果正常重定向到微博的 URL，则说明已经登录
        Ok(None)
    } else {
        // 到达此处，说明网关协议有变化，程序需要进行相应修改
        panic!("网关协议可能已更新，请联系开发者")
    }
}

/// 登录网关
/// - 若返回 `Ok(true)`，登录成功
/// - 若返回 `Ok(false)`，说明登录失败，可能是用户名密码错误，或网关协议有变化
/// - 若返回 `Err(err)`，说明发生了 HTTP 请求错误，需要重试
pub async fn login(
    client: &mut Client<HttpConnector>,
    params: &GxunetParams,
    config: &Config,
) -> hyper::Result<bool> {
    // 在学号前面添加设备类型前缀
    let device_prefix;
    if config.term_type == "1" {
        device_prefix = ",0,"; // PC
    } else {
        device_prefix = ",1,"; // 手机 / 平板
    };
    let device_and_user = format!("{}{}", device_prefix, config.user);

    // 构造请求 URL
    let mut login_url = Url::parse("http://172.17.0.2:801/eportal/").unwrap();
    login_url
        .query_pairs_mut()
        .append_pair("c", "ACSetting")
        .append_pair("a", "Login")
        .append_pair("loginMethod", "1")
        .append_pair("iTermType", &config.term_type)
        .append_pair("wlanuserip", &params.user_ip)
        .append_pair("wlanacip", &params.ac_ip)
        .append_pair("wlanacname", "ME60-1")
        .append_pair("ip", &params.user_ip)
        .append_pair("mac", &params.user_mac)
        .append_pair("DDDDD", &device_and_user)
        .append_pair("upass", &config.password)
        .append_pair("jsVersion", "2.4.3");

    // 发送请求
    let req = http_get(login_url.as_str());
    let resp = client.request(req).await?;
    // 检查 HTTP 状态码是否是 302 Found
    assert_eq!(resp.status(), 302);
    // 获取重定向目标地址
    let location = resp.headers().get("location").unwrap().to_str().unwrap();
    if location.starts_with("http://172.17.0.2:80/3.htm?") {
        // 登录成功！
        Ok(true)
    } else {
        println!("登录失败，跳转至：{}", location);
        Ok(false)
    }
}
