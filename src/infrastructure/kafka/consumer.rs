use futures::StreamExt;
use rdkafka::{
    ClientConfig, Message,
    consumer::{CommitMode, Consumer, StreamConsumer},
};
use serde_json::from_str;

use crate::{
    app::AppState,
    dto::request::viewer_create::ViewerCreateRequest,
};

pub async fn start(app_state: AppState) {
    let consumer = consumer_create();
    consumer_run(consumer, app_state).await;
}

fn consumer_create() -> StreamConsumer {
    let mut config = ClientConfig::new();

    config
        .set("bootstrap.servers", "192.168.0.233:9192")
        .set("group.id", "count-group")
        .set("auto.offset.reset", "earliest");

    let consumer = config.create().expect("Failed to create consumer");

    consumer
}

async fn consumer_run(consumer: StreamConsumer, app_state: AppState) {
    consumer
        .subscribe(&["demo_order_system"])
        .expect("Failed to subscribe to topic");

    while let Some(message) = consumer.stream().next().await {
        match message {
            Ok(msg) => {
                match msg.payload_view::<str>() {
                    None => println!("Message payload is not a string"),
                    Some(Ok(payload)) => {
                        match from_str::<ViewerCreateRequest>(payload) {
                            Ok(deser) => match deser.get_status().as_str() {
                                "Sub" => {
                                    let result =app_state.counter_viewer_usecase.add_viewer_usecase(deser.get_chanel()).await;

                                    match result {
                                        Ok(viewer) => println!("viewer: {:#?}", viewer),
                                        Err(e) => println!("Failed to create viewer: {}", e),
                                    }
                                }
                                "Unsub" => {
                                    let result = app_state
                                        .counter_viewer_usecase
                                        .sub_viewer_usecase(
                                            deser.get_chanel(),
                                        )
                                        .await;

                                    match result {
                                        Ok(viewer) => println!("viewer: {:#?}", viewer),
                                        Err(e) => println!("Failed to delete viewer: {}", e),
                                    }
                                }
                                other => {
                                    eprintln!("Unknown status received: {}", other);
                                }
                            },
                            Err(e) => {
                                eprintln!("Failed to deserialize message payload: {}", e);
                                eprintln!("Raw payload: {}", payload); // in ra luôn để debug
                            }
                        }
                    }
                    Some(Err(e)) => println!("Failed to get message payload: {:?}", e),
                }
                consumer
                    .commit_message(&msg, CommitMode::Async)
                    .expect("Failed to commit message");
            }
            Err(e) => println!("Failed to receive message: {:?}", e),
        }
    }
}
