use crate::depth_update::DepthUpdate;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
mod depth_update;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ws, _) = connect_async("wss://fstream.binance.com/public/ws/btcusdt@depth@100ms")
        .await
        .expect("Failed to connect");

    let (_, mut read) = ws.split();

    while let Some(message) = read.next().await {
        let update: DepthUpdate = serde_json::from_str(message?.to_text()?)?;
        println!("{:?}", update)
    }

    Ok(())
}
