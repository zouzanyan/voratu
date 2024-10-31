use crate::config::read_config::{read_config, BinanceAccountConfig};
use crate::util::magic_util::get_current_time_mills;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use std::error::Error;
use std::time::Duration;

pub struct BinanceClient {
    binance_config: BinanceAccountConfig,
    client: reqwest::Client,
}


impl BinanceClient {
    pub fn new() -> Self {
        let result = read_config().expect("读取yml配置文件错误");
        Self {
            binance_config: result.app.binance,
            client: reqwest::Client::new(),
        }
    }

    /// 赚币账户信息
    pub async fn get_simple_earn_account_info(&self) -> Result<String, Box<dyn Error>> {
        let current_time = get_current_time_mills();
        println!("当前时间戳: {}", current_time);
        let query_string = format!("timestamp={}", current_time);
        let mut hmac = Hmac::new(Sha256::new(), self.binance_config.api_secret().as_bytes());
        hmac.input(query_string.as_bytes());
        let signature: String = hmac.result().code().iter().map(|b| { format!("{:02x}", b) }).collect();
        let url = format!("{}/sapi/v1/simple-earn/account?{}&signature={}", self.binance_config.base_url(), query_string, signature);
        println!("{}", url);
        let res = self.client.get(url)
            .timeout(Duration::from_secs(10))
            .header("X-MBX-APIKEY", self.binance_config.api_key())
            .send()
            .await.unwrap();
        let string = res.text().await.unwrap();
        Ok(string)
    }

    /// 现货账户信息
    pub async fn get_spot_account_info(&self) -> Result<String, Box<dyn Error>> {
        let current_time = get_current_time_mills();
        println!("当前时间戳: {}", current_time);
        let query_string = format!("omitZeroBalances={}&timestamp={}", true, current_time);
        let mut hmac = Hmac::new(Sha256::new(), self.binance_config.api_secret().as_bytes());
        hmac.input(query_string.as_bytes());
        let signature: String = hmac.result().code().iter().map(|b| { format!("{:02x}", b) }).collect();
        let url = format!("{}/api/v3/account?{}&signature={}", self.binance_config.base_url(), query_string, signature);
        println!("{}", url);
        let res = self.client.get(url)
            .timeout(Duration::from_secs(10))
            .header("X-MBX-APIKEY", self.binance_config.api_key())
            .send()
            .await.unwrap();
        let string = res.text().await.unwrap();
        Ok(string)
    }
}
