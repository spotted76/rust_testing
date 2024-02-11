use std::time::Duration;

use futures::prelude::*;
use tokio_stomp::*;

use protobuf::{EnumOrUnknown, Message};

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use example::{get_response, GetRequest, GetResponse};


// You can start a simple STOMP server with docker:
// `docker run -p 61613:61613 rmohr/activemq:latest`

async fn client(sends: &str, msg: &mut GetRequest) -> Result<(), anyhow::Error> {
    let mut conn = client::connect(
        "172.17.0.3:61613",
        "/".to_string(),
        "guest".to_string().into(),
        "guest".to_string().into(),
    )
    .await?;

    let mut iter_count = 1;

    loop {

        msg.age = iter_count;
        let vec: Vec<u8> = msg.write_to_bytes().unwrap();

        println!("Sending iter:  {}", msg.age);
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


    // Encode example request
    let mut msg_one = GetRequest::new();
    msg_one.name = "John Smith".to_string();
    msg_one.age = 0;
    msg_one.features.push("one".to_string());
    msg_one.features.push("two".to_string());
    

    let fut1 = Box::pin(client("Topic", &mut msg_one));

    fut1.await

}