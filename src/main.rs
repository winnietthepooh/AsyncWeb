mod hypixel_requests;
mod auction;
use reqwest::Client;


extern crate core;
use futures::{stream, StreamExt};

#[tokio::main]
async fn main() {
    let init = hypixel_requests::request().await;
    if init.is_none() {
        return;
    }
    let init = init.unwrap();

    if !init.success {
        return;
    }
    let client = Client::new();
    let mut urls:Vec<String> = vec![];
    let sky_url: String = String::from("https://api.hypixel.net/skyblock/auctions");

    for x in 1..init.total_pages {
        let url = format!("{}?page={}", sky_url.clone(), x.clone());
        urls.push(url)
    }

    let bodies = stream::iter(urls.clone())
        .map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url).send().await?;
                println!("{:?}",resp.status());
                resp.json::<auction::Root>().await
            }
        })
        .buffer_unordered(urls.len());

    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => println!("Got page: {}", b.page),
                Err(e) => eprintln!("Got an error: {}", e),
            }
        })
        .await;

}
