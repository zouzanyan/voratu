use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use serde_json::Value;
use crate::config::read_config::{read_config, BinanceAccountConfig};

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

    pub async fn get_account_info(&self) -> Result<(), Box<dyn Error>> {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        println!("{}", current_time);
        let query_string = format!("timestamp={}", current_time);
        let mut hmac = Hmac::new(Sha256::new(), self.binance_config.api_secret().as_bytes());
        hmac.input(query_string.as_bytes());
        let signature: String = hmac.result().code().iter().map(|b| { format!("{:02x}", b) }).collect();
        let url = format!("{}/api/v3/account?timestamp={}&signature={}", self.binance_config.base_url(), current_time, signature);
        println!("{}", url);
        let res = self.client.get(url)
            .timeout(Duration::from_secs(10))
            .header("X-MBX-APIKEY", self.binance_config.api_key())
            .send()
            .await.unwrap();
        // println!("{:?}", &res.text().await.unwrap());
        let result: Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();
        println!("{}", &result["balances"]);
        Ok(())
    }
}