use std::sync::Arc;

use crate::{depth_update::DepthUpdate, orderbook::Orderbook, snapshot::Snapshot};
use futures_util::StreamExt;
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;
mod depth_update;
mod orderbook;
mod snapshot;
mod types;
use std::error::Error;

const SNAPSHOT_URL: &str = "https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=1000";
const DEPTH_URL: &str = "wss://fstream.binance.com/public/ws/btcusdt@depth@100ms";

async fn create_orderbook(snapshot_url: &str) -> anyhow::Result<Orderbook> {
    let snapshot = reqwest::get(snapshot_url).await?.json::<Snapshot>().await?;

    Ok(Orderbook::from(snapshot))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let book: Arc<Mutex<Option<Orderbook>>> = Arc::new(Mutex::new(None));
    let update_buffer: Arc<Mutex<Vec<DepthUpdate>>> = Arc::new(Mutex::new(Vec::new()));

    let (ws, _) = connect_async(DEPTH_URL).await.expect("Failed to connect");

    let (_, mut read) = ws.split();

    tokio::spawn({
        let shared_book = Arc::clone(&book);
        let shared_buffer = Arc::clone(&update_buffer);
        async move {
            let mut book_from_snapshot = create_orderbook(SNAPSHOT_URL).await?;

            let buffer = std::mem::take(&mut *shared_buffer.lock().await);

            for update in buffer {
                book_from_snapshot.apply_depth_update(update);
            }

            let mut book = shared_book.lock().await;
            *book = Some(book_from_snapshot);

            anyhow::Ok(())
        }
    });

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

        let mut book = book.lock().await;

        match &mut *book {
            None => {
                let mut buffer = update_buffer.lock().await;
                buffer.push(depth_update);
            }
            Some(book) => {
                book.apply_depth_update(depth_update);
                println!("{}", book.display_top_levels(20));
            }
        }
    }

    Ok(())
}
