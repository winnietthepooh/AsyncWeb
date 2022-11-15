mod others;

use std::collections::HashSet;
use std::hash::Hash;
use reqwest::Client;

extern crate core;
use futures::future::join_all;
use crate::others::auction::{Auction, Root};
use crate::others::hypixel_requests;

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
    let mut urls: Vec<String> = vec![];
    let sky_url: String = String::from("https://api.hypixel.net/skyblock/auctions");
    for x in 1..init.clone().total_pages {
        let url = format!("{}?page={}", sky_url.clone(), x.clone());
        urls.push(url)
    }
    let bodies = join_all(urls.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.json::<Root>().await
        }
    }))
        .await;

    let mut data: Vec<Root> = vec![init.clone()];

    for b in bodies {
        match b {
            Ok(b) => {
                data.push(b.clone());
            },
            Err(e) => eprintln!("Got an error: {}", e),
        }
    }
    let mut auctions: Vec<Auction> = vec![];
    for x in data {
        auctions.extend(x.auctions)
    }
    println!("{}", auctions.clone().len());
    println!("{}", init.total_auctions);

    println!("has unique elements: {}", has_unique_elements(auctions))

}

fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

// ur  mmo - Thayne Tubbs
