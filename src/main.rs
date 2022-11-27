use reqwest::{Client};
use serde_json::Value;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::time::{SystemTime, Duration};
use std::thread;

type HmacSha256 = Hmac<Sha256>;

const API_KEY: &str = "";
const SECRET_KEY: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let client: Client = Client::new();

    for i in 0..550 {

        let res1 = do_work("SELL", &client).await;
        thread::sleep(Duration::from_millis(1000));
        let res2 = do_work("BUY", &client).await;
        thread::sleep(Duration::from_millis(1000));
        
        println!("iteration: {}, 1st trans: {:?}, 2nd trans: {:?}", i, res1, res2);
    }
    Ok(())
}

async fn get_signature(to_sign: String) -> String {

    let mut mac = HmacSha256::new_from_slice(SECRET_KEY.as_bytes()).expect("by crose");
    mac.update(to_sign.as_bytes());
    let result = mac.finalize();
    let res = result.into_bytes();

    let mut signature: Vec<u8> = vec![];
    for i in 0..32{
        signature.push(res[i]);
    }

    let signed: String = hex::encode(signature);
    signed
}

async fn get_sys_time_in_secs() -> String {

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_millis().to_string(),
        Err(_) => panic!("DSadasdsa"),
    }
}

async fn get_quanity(client: &Client, side: &str) -> Result<String, Box<dyn std::error::Error>> {

    let asset: String = String::from(if side == "SELL" {
        "BUSD"
    } else {
        "ZAR"
    });

    let timestamp: String = get_sys_time_in_secs().await;

    let uri: String = format!("asset={}&timestamp={}", asset, timestamp);
    let uri2: String = uri.clone();

    let signature: String = get_signature(uri).await;
    let url: String = format!("https://api.binance.com/sapi/v3/asset/getUserAsset?{}&signature={}", uri2, signature);
    
    let quantity: String = client.post(url)
    .header("X-MBX-APIKEY", API_KEY)
    .send()
    .await?
    .text()
    .await?;

    Ok(quantity)
}

async fn get_uri(to_do: &str, quantity: String) -> String {

    let timestamp: String = get_sys_time_in_secs().await;

        let uri: String  = format!("symbol=BUSDZAR&side={}&type=MARKET&quantity=100&recvWindow=60000&timestamp={}", to_do, timestamp);
        let uri2: String = uri.clone();
    
        let signature: String = get_signature(uri).await;
        let signed_uri: String = format!("{}&signature={}", uri2, signature);
    
        return signed_uri


}

async fn do_work(to_do: &str, client: &Client) -> Result<(), Box<dyn std::error::Error>>{
    let quant: String = get_quanity(&client, to_do).await.unwrap().to_string();
    let quantity: Value = serde_json::from_str(&quant)?;


    let uri: String = get_uri(to_do, quantity[0]["free"].to_string().replace('"', "")).await;
    let url: String = format!("https://api.binance.com/api/v3/order?{}", uri);

    let response: String = client.post(url)
    .header("X-MBX-APIKEY", API_KEY)
    .send()
    .await?
    .text()
    .await?;

    println!("{:?}", response);
    Ok(())
}
