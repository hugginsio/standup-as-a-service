// Adapted from https://github.com/rabbitmq/rabbitmq-tutorials/
// SPDX-License-Identifier: Apache-2.0
// Changes:
// - Use tracing instead of println
// - Modify queue names and credentials
// - Process messages and insert to database

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicConsumeArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
};
use std::str;
use tokio::{self};
use tokio::{io::Error as TError, sync::Notify};

#[tokio::main]
async fn main() -> Result<(), Box<TError>> {
    tracing_subscriber::fmt::init();
    tracing::info!("attempting rmq connection");
    let conn = Connection::open(&OpenConnectionArguments::new(
        "rabbitmq.standup.svc.cluster.local",
        5672,
        "user",
        "b9R09B7edSRQpZxz",
    ))
    .await
    .unwrap();

    conn.register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    let ch = conn.open_channel(None).await.unwrap();
    ch.register_callback(DefaultChannelCallback).await.unwrap();

    let q_args = QueueDeclareArguments::default()
        .queue(String::from("work-items"))
        .finish();
    let (queue_name, _, _) = ch.queue_declare(q_args).await.unwrap().unwrap();
    let consumer_args = BasicConsumeArguments::new(&queue_name, "listener-rs");
    let (_ctag, mut rx) = ch.basic_consume_rx(consumer_args).await.unwrap();
    tracing::info!("waiting for messages");

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Some(payload) = msg.content {
                tracing::info!("rcvd: {:?}", str::from_utf8(&payload).unwrap());
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;

    Ok(())
}
