use serde_json::Value;

mod config;
mod util;

#[tokio::main]
async fn main() {
    let client = config::binance_client::BinanceClient::new();
    let res = client.get_simple_earn_account_info().await.unwrap();
    let result: Value = serde_json::from_str(&res).unwrap();
    let result1 = result["totalAmountInBTC"].as_str().unwrap().parse::<f64>().unwrap();
    println!("{}", result1);
    let x = client.get_spot_account_info().await.unwrap();
    println!("{}", x);

    // let mut string = String::from("");
    // io::stdin().read_line(&mut string).unwrap();
}


