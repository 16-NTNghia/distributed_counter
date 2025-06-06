use crate::dto::request::{viewer_create::ViewerCreateRequest};
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use serde_json::to_string;
use std::error::Error;
use std::time::Duration;

pub async fn start(message: ViewerCreateRequest) -> Result<(), Box<dyn Error>> {
    let producer = producer_create()?;
    producer_run(producer, message).await?;
    Ok(())
}

fn producer_create() -> Result<FutureProducer, Box<dyn Error>> {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "192.168.0.233:9192");
    let producer = config.create()?;
    Ok(producer)
}

async fn producer_run(
    producer: FutureProducer,
    message: ViewerCreateRequest,
) -> Result<(), Box<dyn Error>> {
    // println!("payload in producer: {:?}", message);

    let json_payload = to_string(&message)?;

    // Không cần key vẫn hợp lệ
    let record: FutureRecord<String, String> =
        FutureRecord::to("demo_order_system").payload(&json_payload);

    // println!("record in producer: {:?}", record);

    let delivery_status = producer
        .send(record, Timeout::After(Duration::from_millis(5000)))
        .await;

    match delivery_status {
        Ok(delivery) => {
            // println!("Message sent successfully: {:?}", delivery);
            Ok(())
        }
        Err((err, _msg)) => {
            // eprintln!("Failed to send message: {:?}", err);
            Err(Box::new(err))
        }
    }
}
