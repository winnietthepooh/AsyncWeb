use crate::auction;
use crate::auction::Root;
use reqwest::Client;

pub async fn request() -> Option<Root> {
    let client = Client::new();

    let resp = client.get("https://api.hypixel.net/skyblock/auctions").send().await.unwrap();
    println!("{:?}",resp.status());
    let data = resp.json::<auction::Root>().await.expect("AHHHH also this error is in hypxiel_requests");
    Some(data)
}
