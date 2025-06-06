use scylla::client::{session::Session, session_builder::SessionBuilder};

use crate::config::env_config::get_env;



pub async fn create_session() -> Session{
    let uri = get_env("SCYLLA_URI");
    let keyspace = get_env("SCYLLA_KEYSPACE");

    let session = SessionBuilder::new()
        .known_node(uri)
        .use_keyspace(keyspace, false)
        .build()
        .await
        .expect("Failed to create session");

    println!("Connected scyllaDB successfully");

    session
}