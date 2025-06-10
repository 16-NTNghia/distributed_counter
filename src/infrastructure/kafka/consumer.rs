use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicI64, Ordering},
    },
    time::Duration,
};

use dashmap::DashMap;
use futures::StreamExt;
use rdkafka::{
    ClientConfig, Message,
    consumer::{CommitMode, Consumer, StreamConsumer},
};
use serde_json::from_str;
use tokio::{
    spawn,
    time::{self, interval},
};

use crate::{app::AppState, dto::request::viewer_create::ViewerCreateRequest};

pub async fn start(app_state: AppState) {
    let consumer = consumer_create();

    let channel_map: Arc<DashMap<String, Arc<AtomicI64>>> = Arc::new(DashMap::new());

    let map_clone = channel_map.clone();

    let app_state_clone = app_state.clone();

    spawn(async move {
        let mut interval = interval(Duration::from_millis(500));

        loop {
            interval.tick().await;

            let snapshot: Vec<(String, i64)> = map_clone
                .iter()
                .map(|entry| (entry.key().clone(), entry.value().load(Ordering::SeqCst)))
                .collect();

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .expect("Failed to open file");

            for (channel, count) in &snapshot {
                // writeln!(file, "Viewer count of channel {}: {}", channel, count)
                //     .expect("Failed to write to file");

                if let Err(e) = app_state_clone
                    .counter_viewer_usecase
                    .update_viewer_usecase(*count, channel.to_string())
                    .await
                {
                    eprintln!(
                        "Failed to update viewer usecase for channel {}: {}",
                        channel, e
                    );
                }
            }
            // 2. Reset lại tất cả count về 0
            for (channel, old_value) in snapshot {
                if let Some(entry) = map_clone.get(&channel) {
                    entry.fetch_sub(old_value, Ordering::SeqCst);
                }
            }
        }
    });

    consumer_run(consumer, channel_map).await;
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

async fn consumer_run(consumer: StreamConsumer, channel_map: Arc<DashMap<String, Arc<AtomicI64>>>) {
    consumer
        .subscribe(&["demo_order_system"])
        .expect("Failed to subscribe to topic");

    while let Some(message) = consumer.stream().next().await {
        match message {
            Ok(msg) => {
                match msg.payload_view::<str>() {
                    None => println!("Failed to get message payload"),
                    Some(Ok(payload)) => {
                        match from_str::<ViewerCreateRequest>(payload) {
                            Ok(deser) => {
                                let channel = deser.get_channel().to_string();

                                let counter = channel_map
                                    .entry(channel)
                                    .or_insert_with(|| Arc::new(AtomicI64::new(0)));

                                match deser.get_status().as_str() {
                                    "Sub" => {
                                        // let result = app_state
                                        //     .counter_viewer_usecase
                                        //     .add_viewer_usecase(deser.get_chanel())
                                        //     .await;
                                        counter.fetch_add(1, Ordering::SeqCst);
                                        // match result {
                                        //     Ok(viewer) => println!("viewer: {:#?}", viewer),
                                        //     Err(e) => println!("Failed to create viewer: {}", e),
                                        // }
                                    }
                                    "Unsub" => {
                                        // let result = app_state
                                        //     .counter_viewer_usecase
                                        //     .sub_viewer_usecase(deser.get_chanel())
                                        //     .await;

                                        counter.fetch_sub(1, Ordering::SeqCst);

                                        // match result {
                                        //     Ok(viewer) => println!("viewer: {:#?}", viewer),
                                        //     Err(e) => println!("Failed to delete viewer: {}", e),
                                        // }
                                    }
                                    other => {
                                        eprintln!("Unknown status received: {}", other);
                                    }
                                }
                            }
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
