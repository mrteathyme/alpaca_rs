use std::env;

#[tokio::test]
async fn crypto_historical_bars() -> anyhow::Result<()> { 
    let key = env::var("APCA_KEY").expect("Missing APCA_KEY EnvVar");
    let secret = env::var("APCA_SECRET").expect("Missing APCA_SECRET EnvVar");
    let request = alpaca_rs::data::crypto::historical_bars(&key, &secret, vec!["BTC/USD"], alpaca_rs::data::crypto::Timeframe::Hour(1), None, None, None, None, Some(alpaca_rs::data::crypto::SortDirection::Ascending))?;
    let mut bars = vec![];
    let response = request.send(execute).await?;
    bars.push(response.bars().clone());
    let mut next_page_token = response.next_page_token().cloned();
    while next_page_token.is_some() {
        let request = alpaca_rs::data::crypto::historical_bars(&key, &secret, vec!["BTC/USD"], alpaca_rs::data::crypto::Timeframe::Hour(1), None, None, None, next_page_token.as_deref(), Some(alpaca_rs::data::crypto::SortDirection::Ascending))?;
        let response = request.send(execute).await?;
        bars.push(response.bars().clone());
        next_page_token = response.next_page_token().cloned();
    }
    Ok(()) 
}

#[tokio::test]
async fn crypto_latest_orderbook() -> anyhow::Result<()> {
    let key = env::var("APCA_KEY").expect("Missing APCA_KEY EnvVar");
    let secret = env::var("APCA_SECRET").expect("Missing APCA_SECRET EnvVar");
    !todo!("Finish this api dipshit");
    Ok(())
}

#[tokio::test]
async fn crypto_latest_trades() -> anyhow::Result<()> {
    let key = env::var("APCA_KEY").expect("Missing APCA_KEY EnvVar");
    let secret = env::var("APCA_SECRET").expect("Missing APCA_SECRET EnvVar");
    !todo!("Finish this api dipshit");
    Ok(())
}

#[tokio::test]
async fn crypto_latest_quotes() -> anyhow::Result<()> {
    let key = env::var("APCA_KEY").expect("Missing APCA_KEY EnvVar");
    let secret = env::var("APCA_SECRET").expect("Missing APCA_SECRET EnvVar");
    !todo!("Finish this api dipshit");
    Ok(())
}

#[tokio::test]
async fn crypto_latest_bars() -> anyhow::Result<()> {
    let key = env::var("APCA_KEY").expect("Missing APCA_KEY EnvVar");
    let secret = env::var("APCA_SECRET").expect("Missing APCA_SECRET EnvVar");
    !todo!("Finish this api dipshit");
    Ok(())
}

#[tokio::test]
async fn crypto_historical_quotes() -> anyhow::Result<()> {
    let key = env::var("APCA_KEY").expect("Missing APCA_KEY EnvVar");
    let secret = env::var("APCA_SECRET").expect("Missing APCA_SECRET EnvVar");
    !todo!("Finish this api dipshit");
    Ok(())
}

#[tokio::test]
async fn crypto_snapshots() -> anyhow::Result<()> {
    let key = env::var("APCA_KEY").expect("Missing APCA_KEY EnvVar");
    let secret = env::var("APCA_SECRET").expect("Missing APCA_SECRET EnvVar");
    !todo!("Finish this api dipshit");
    Ok(())
}

#[tokio::test]
async fn crypto_historical_trades() -> anyhow::Result<()> {
    let key = env::var("APCA_KEY").expect("Missing APCA_KEY EnvVar");
    let secret = env::var("APCA_SECRET").expect("Missing APCA_SECRET EnvVar");
    !todo!("Finish this api dipshit");
    Ok(())
}


async fn execute(request: http::Request<String>) -> anyhow::Result<bytes::Bytes> {
    let client = reqwest::Client::new();
    let response = client.execute(request.try_into()?).await?;
    let bytes = response.bytes().await?;
    let object = serde_json::from_slice::<serde_json::Value>(&bytes)?;
    println!("{:#?}", object);
    Ok(bytes)
}
