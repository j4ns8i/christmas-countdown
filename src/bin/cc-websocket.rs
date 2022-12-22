use std::env;

use futures_util::{StreamExt, future};
// use futures_util::{future, StreamExt};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize)]
struct WebSocketOpenResponse {
    ok: bool,
    error: Option<String>,
    url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bearer_token =
        env::var("CC_WEBSOCKET_BEARER_TOKEN").or(Err("Must provide CC_WEBSOCKET_BEARER_TOKEN"))?;
    let resp: WebSocketOpenResponse = reqwest::Client::new()
        .post("https://slack.com/api/apps.connections.open")
        .header(CONTENT_TYPE, "application/json")
        .bearer_auth(bearer_token)
        .send()
        .await?
        .json()
        .await?;

    if let Some(err) = resp.error {
        return Err(err.into());
    }

    let url = match resp.url {
        Some(u) => url::Url::parse(&u).expect("invalid url"),
        None => return Err("no websocket url provided".into()),
    };

    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await.expect("failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (_write, read) = ws_stream.split();
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    let _a = ws_to_stdout.await;

    // println!("response: {:?}", resp);
    Ok(())
}
