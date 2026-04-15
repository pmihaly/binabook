use crate::{
    depth_update::DepthUpdate,
    orderbook::{Orderbook, OrderbookEvent},
    snapshot::Snapshot,
};
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
mod depth_update;
mod orderbook;
mod snapshot;
mod types;

const SNAPSHOT_URL: &str = "https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=1000";
const DEPTH_URL: &str = "wss://fstream.binance.com/public/ws/btcusdt@depth@100ms";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (ws, _) = connect_async(DEPTH_URL).await.expect("Failed to connect");

    let (_, mut read) = ws.split();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<OrderbookEvent>(100);

    tokio::spawn({
        let tx_thread = tx.clone();
        async move {
            let snapshot = reqwest::get(SNAPSHOT_URL).await?.json::<Snapshot>().await?;

            tx_thread
                .send(OrderbookEvent::SnapshotUpdate(snapshot))
                .await?;

            Ok::<(), anyhow::Error>(())
        }
    });

    tokio::spawn({
        let tx_thread = tx.clone();
        async move {
            while let Some(message) = read.next().await {
                let msg = message?;
                let text = msg.to_text()?;
                let depth_update = match serde_json::from_str::<DepthUpdate>(&text.to_string()) {
                    Err(err) => {
                        println!("message dropped: {}, msg: {}", err, msg);
                        continue;
                    }
                    Ok(update) => update,
                };

                tx_thread
                    .send(OrderbookEvent::DepthUpdate(depth_update))
                    .await?;
            }

            Ok::<(), anyhow::Error>(())
        }
    });

    let mut orderbook: Option<Orderbook> = None;
    let mut update_buffer: Vec<DepthUpdate> = Vec::new();
    while let Some(event) = rx.recv().await {
        match (&mut orderbook, event) {
            (_, OrderbookEvent::SnapshotUpdate(snapshot)) => {
                let mut book = Orderbook::from(snapshot);
                for buffered in update_buffer.drain(..) {
                    book.apply_depth_update(buffered);
                }
                orderbook = Some(book);
            }
            (None, OrderbookEvent::DepthUpdate(update)) => update_buffer.push(update),
            (Some(book), OrderbookEvent::DepthUpdate(update)) => {
                book.apply_depth_update(update);
                println!("{}", book.display_top_levels(20));
            }
        }
    }

    Ok(())
}
