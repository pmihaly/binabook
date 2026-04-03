use crate::depth_update::DepthUpdate;
use futures_util::{StreamExt, TryStreamExt};
use tokio_tungstenite::connect_async;
mod depth_update;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ws, _) = connect_async("wss://fstream.binance.com/public/ws/btcusdt@depth@100ms")
        .await
        .expect("Failed to connect");

    let (_, read) = ws.split();

    read.map(|message| -> anyhow::Result<DepthUpdate> {
        Ok(serde_json::from_str(message?.to_text()?)?)
    })
    .try_for_each(|update| async move {
        println!("{update:?}");
        Ok(())
    })
    .await?;

    Ok(())
}
