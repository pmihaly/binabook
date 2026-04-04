use std::sync::Arc;

use crate::{depth_update::DepthUpdate, orderbook::Orderbook, snapshot::Snapshot};
use futures_util::{StreamExt, TryStreamExt};
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;
mod depth_update;
mod orderbook;
mod snapshot;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body: Snapshot =
        reqwest::get("https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=1000")
            .await?
            .json()
            .await?;
    let book = Arc::new(Mutex::new(Orderbook::from(body)));

    let (ws, _) = connect_async("wss://fstream.binance.com/public/ws/btcusdt@depth@100ms")
        .await
        .expect("Failed to connect");

    let (_, read) = ws.split();

    read.map(|message| -> anyhow::Result<DepthUpdate> {
        Ok(serde_json::from_str(message?.to_text()?)?)
    })
    .try_for_each(|update| {
        let shared_book = Arc::clone(&book);
        async move {
            let mut guard = shared_book.lock().await;
            guard.apply_depth_update(update);

            println!("{}", guard.display_top_levels(10));

            Ok(())
        }
    })
    .await?;

    Ok(())
}
