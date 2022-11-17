
use reqwest::Client;

use crate::others::auction::Root;

pub async fn request() -> Option<Root> {
    let client = Client::new();

    let resp = client.get("http://api.hypixel.net/skyblock/auctions").header(reqwest::header::USER_AGENT, "WinnietThePooh rust program").send().await;
    let resp = match resp {
        Ok(Res) => Res,
        Err(err) => {
            println!("Error: {}", err);
            return None
        }
    };
    let data = resp.json::<Root>().await.unwrap();
    Some(data)
}
