
use reqwest::Client;

use crate::others::auction::Root;

pub async fn request() -> Option<Root> {
    let client = Client::new();

    let resp = client.get("https://api.hypixel.net/skyblock/auctions").send().await.unwrap();
    let data = resp.json::<Root>().await.unwrap();
    Some(data)
}
