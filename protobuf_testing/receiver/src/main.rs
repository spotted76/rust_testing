use std::time::Duration;

use futures::prelude::*;
use tokio_stomp::*;

use proto_msg_lib::msg_one;

use quick_protobuf::{MessageRead, BytesReader};
use quick_protobuf::writer;
use quick_protobuf::reader;

// This examples consists of two futures, each of which connects to a local server,
// and then sends either PING or PONG messages to the server while listening
// for replies. This continues indefinitely (ctrl-c to exit)

// You can start a simple STOMP server with docker:
// `docker run -p 61613:61613 rmohr/activemq:latest`

async fn client(listens: &str) -> Result<(), anyhow::Error> {
    let mut conn = client::connect(
        "172.17.0.3:61613",
        "/".to_string(),
        "guest".to_string().into(),
        "guest".to_string().into(),
    )
    .await?;

    conn.send(client::subscribe(listens, "myid")).await?;


    loop {

        let msg = conn.next().await.transpose()?;
        if let Some(FromServer::Message { body, .. }) = msg.as_ref().map(|m| &m.content) {
            let reconstructed : msg_one::MessageOne = reader::deserialize_from_slice(&body.as_ref().unwrap()).expect("Cannot convert into a `MessageOne`");
            println!("Received:  {:?}", reconstructed);
        } else {
            anyhow::bail!("Unexpected: {:?}", msg)
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let fut1 = Box::pin(client("Topic"));
    fut1.await

}