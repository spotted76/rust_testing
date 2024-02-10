use std::time::Duration;

use futures::prelude::*;
use tokio_stomp::*;

use proto_msg_lib::msg_one::{self, MessageOne};

use quick_protobuf::{MessageRead, BytesReader};
use quick_protobuf::writer;
use quick_protobuf::reader;

// This examples consists of two futures, each of which connects to a local server,
// and then sends either PING or PONG messages to the server while listening
// for replies. This continues indefinitely (ctrl-c to exit)

// You can start a simple STOMP server with docker:
// `docker run -p 61613:61613 rmohr/activemq:latest`

async fn client(sends: &str, msg: &mut MessageOne) -> Result<(), anyhow::Error> {
    let mut conn = client::connect(
        "172.17.0.3:61613",
        "/".to_string(),
        "guest".to_string().into(),
        "guest".to_string().into(),
    )
    .await?;

    let mut iter_count = 1;

    loop {

        let mut msg = msg_one::MessageOne{..Default::default()};
        msg.f_int32 = Some(iter_count);

        let vec = writer::serialize_into_vec(&msg).expect("Cannot serialize `MessageOne`");

        println!("Sending iter:  {}", msg.f_int32.unwrap());
        conn.send(
            ToServer::Send {
                destination: sends.into(),
                transaction: None,
                // body: Some(msg.to_vec()),
                body: Some(vec),
            }
            .into(),
        )
        .await?;

        iter_count += 1;

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    let mut msg_one = MessageOne{..Default::default()};

    let fut1 = Box::pin(client("Topic", &mut msg_one));

    fut1.await

}