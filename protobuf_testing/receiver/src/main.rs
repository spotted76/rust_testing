use std::{fmt::{Debug, Display}, time::Duration};

use futures::prelude::*;
use tokio_stomp::*;

use protobuf::Message;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use example::GetRequest;

// You can start a simple STOMP server with docker:
// `docker run -p 61613:61613 rmohr/activemq:latest`

async fn client<T>(listens: &str) -> Result<(), anyhow::Error> 
    where T: protobuf::Message + Debug {
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
            let reconstructed = T::parse_from_bytes(&body.as_ref().unwrap()).unwrap();
            // let reconstructed : msg_one::MessageOne = reader::deserialize_from_slice(&body.as_ref().unwrap()).expect("Cannot convert into a `MessageOne`");
            println!("Received:  {:#?}", reconstructed);
        } else {
            anyhow::bail!("Unexpected: {:?}", msg)
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    //Perfect!!!  This is now working with a template.  So..  I envision the ability to spin up as many of these
    //futures as desired based on configuration. 
    let fut1 = Box::pin(client::<example::GetRequest>("Topic"));
    fut1.await

}
