use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::Range;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.hypixel.net/auctions/?page=0")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let test = match find_pages(&resp) {
        Some(x) => x,
        None => return Ok(())
    };
    let total_pages = match test {
        Ok(x) => x,
        Err(_) => return Ok(())
    };

}

fn find_pages(map: &HashMap<String, String>) -> Option<Result<i32, ParseIntError>> {
    map.iter()
        .find_map(|key| if key.0 == &String::from("total_pages") { Some(key.1.parse()) } else { None })
}

