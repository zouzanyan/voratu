use std::io;

mod config;


#[tokio::main]
async fn main() {
    let client = config::binance_client::BinanceClient::new();
    client.get_account_info().await.unwrap();
    let mut string = String::from("");
    io::stdin().read_line(&mut string).unwrap();
}


