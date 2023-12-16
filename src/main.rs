use serde_json::Value;

const KEY: &str = "bppat97rh5reoatojn80";

fn main() {
    let json = get_info(&"apple");
    println!("{}",json["count"]);
}

#[tokio::main]
async fn get_info(ticker: &str) -> Value{
    let response = reqwest::get(format!("https://finnhub.io/api/v1/search?q={}&token={}",ticker,KEY))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let json: Value = serde_json::from_str(&response).unwrap();
    return json;
}
