use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ws, _) = connect_async("wss://fstream.binance.com/public/ws/btcusdt@depth@100ms")
        .await
        .expect("Failed to connect");

    let (_, mut read) = ws.split();

    while let Some(message) = read.next().await {
        println!("{}", message?.to_text()?)
    }

    Ok(())
}
