use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.get("https://httpbin.org/ip").send().await?;
    let text = res.json::<HashMap<String, String>>().await?;
    println!("{:#?}", text);
    Ok(())
}
