// Adapted from https://github.com/rabbitmq/rabbitmq-tutorials/
// SPDX-License-Identifier: Apache-2.0
// Changes:
// - Use tracing instead of println
// - Modify queue names and credentials
// - Desearialize JSON messages to struct(s) and insert to database(s)

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicAckArguments, BasicConsumeArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
};
use core::str;
use serde::{Deserialize, Serialize};
use tokio::{io::Error as TError, sync::Notify};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkItem {
    pub fields: Fields,
    pub id: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    #[serde(rename = "JBH.InstallDate")]
    pub jbh_install_date: String,
    #[serde(default, rename = "Microsoft.VSTS.Common.Risk")]
    pub microsoft_vsts_common_risk: String,
    #[serde(rename = "Microsoft.VSTS.Scheduling.StoryPoints")]
    pub microsoft_vsts_scheduling_story_points: f32,
    #[serde(rename = "System.AssignedTo")]
    pub system_assigned_to: SystemAssignedTo,
    #[serde(rename = "System.State")]
    pub system_state: String,
    #[serde(rename = "System.Title")]
    pub system_title: String,
    #[serde(rename = "System.WorkItemType")]
    pub system_work_item_type: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemAssignedTo {
    pub display_name: String,
    pub unique_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorkItemDocument {
    pub identifier: i32,
    pub item_type: String,
    pub title: String,
    pub assigned_to: String,
    pub state: String,
    pub install_date: String,
    pub points: String,
}

#[derive(Default, Deserialize, Serialize)]
struct Flag {
    flag: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<TError>> {
    tracing_subscriber::fmt::init();

    tracing::info!("attempting rmq connection");
    let rabbitmq_conn = Connection::open(&OpenConnectionArguments::new(
        "rabbitmq.standup.svc.cluster.local",
        5672,
        "user",
        "rabbit",
    ))
    .await
    .unwrap();

    rabbitmq_conn
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    let ch = rabbitmq_conn.open_channel(None).await.unwrap();
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
                let payload_string = str::from_utf8(&payload).unwrap();
                let item: WorkItem = serde_json::from_str(payload_string).unwrap_or_else(|e| {
                    tracing::warn!("failed to deserialize: {}", e);
                    WorkItem::default()
                });

                let document = WorkItemDocument {
                    identifier: item.id,
                    item_type: item.fields.system_work_item_type,
                    title: item.fields.system_title,
                    assigned_to: item.fields.system_assigned_to.unique_name,
                    state: item.fields.system_state,
                    install_date: item.fields.jbh_install_date,
                    points: item
                        .fields
                        .microsoft_vsts_scheduling_story_points
                        .to_string(),
                };

                tracing::info!("rcvd: {:?}", &document);

                ch.basic_ack(BasicAckArguments::new(
                    msg.deliver.unwrap().delivery_tag(),
                    false,
                ))
                .await
                .unwrap();

                // calculate the flag value anyway in case the request fails
                let mut feature_flag: bool = item.id % 2 == 0;
                let flag_request = reqwest::get(format!(
                    "http://feature-flags.standup.svc.cluster.local:3000/where-do-i-put/{}",
                    item.id
                ));

                match flag_request.await {
                    Ok(response_raw) => match response_raw.json::<Flag>().await {
                        Ok(response) => feature_flag = response.flag,
                        Err(e) => {
                            tracing::error!("failed to get flag: {:?}", e)
                        }
                    },
                    Err(e) => {
                        tracing::error!("failed to get flag: {:?}", e)
                    }
                }

                let http = reqwest::Client::new();
                if feature_flag {
                    match http
                        .post("http://standup-api.standup.svc.cluster.local:3000/work-items/modern")
                        .json(&document)
                        .send()
                        .await
                    {
                        Ok(response) => tracing::info!("response: {:#?}", response),
                        Err(e) => tracing::info!("failure: {:#?}", e),
                    }
                } else {
                    match http
                        .post("http://standup-api.standup.svc.cluster.local:3000/work-items/legacy")
                        .json(&document)
                        .send()
                        .await
                    {
                        Ok(response) => tracing::info!("response: {:#?}", response),
                        Err(e) => tracing::info!("failure: {:#?}", e),
                    }
                }
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;

    Ok(())
}
