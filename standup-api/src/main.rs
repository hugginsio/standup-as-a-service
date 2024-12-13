use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorkItem {
    pub identifier: i32,
    pub item_type: String,
    pub title: String,
    pub assigned_to: String,
    pub state: String,
    pub install_date: String,
    pub points: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/work-items/all", get(all_work_items))
        .route("/work-items/modern", post(modern_upsert))
        .route("/work-items/legacy", post(legacy_upsert))
        .fallback(handle_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("started on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn common_upsert(collection: Collection<WorkItem>, payload: WorkItem) {
    let existing = collection
        .find_one(doc! { "identifier": payload.identifier})
        .await;

    match existing {
        Err(e) => {
            tracing::error!("mongodb query failed: {:?}", e)
        }
        Ok(document) => {
            if document.is_none() {
                tracing::info!("existing record not found, inserting");
                match collection.insert_one(payload).await {
                    Ok(r) => {
                        tracing::info!("inserted {:?}", r.inserted_id)
                    }
                    Err(e) => {
                        tracing::error!("insert failed {:?}", e)
                    }
                }
            } else {
                tracing::info!("existing record found, updating");
                let filter = doc! { "identifier": payload.identifier};
                let update = doc! {"$set": doc! {
                    "item_type": payload.item_type,
                    "title": payload.title,
                    "assigned_to": payload.assigned_to,
                    "state": payload.state,
                    "install_date": payload.install_date,
                    "points": payload.points
                }};

                match collection.update_one(filter, update).await {
                    Ok(r) => {
                        tracing::info!("updated {:?}", r.upserted_id)
                    }
                    Err(e) => {
                        tracing::error!("update failed {:?}", e)
                    }
                }
            }
        }
    }
}

async fn legacy_upsert(Json(payload): Json<WorkItem>) -> StatusCode {
    tracing::info!("processing {:?}", &payload);
    let connection = Client::with_uri_str("mongodb://standup:mongodb@standup-legacy-0.standup-legacy-svc.standup.svc.cluster.local:27017/admin?replicaSet=standup-legacy&ssl=false").await.unwrap();
    let collection: Collection<WorkItem> = connection.database("standup").collection("work-items");
    common_upsert(collection, payload).await;

    StatusCode::OK
}

async fn modern_upsert(Json(payload): Json<WorkItem>) -> StatusCode {
    tracing::info!("processing {:?}", &payload);
    let connection = Client::with_uri_str("mongodb://standup:mongodb@standup-modern-0.standup-modern-svc.standup.svc.cluster.local:27017/admin?replicaSet=standup-modern&ssl=false").await.unwrap();
    let collection: Collection<WorkItem> = connection.database("standup").collection("work-items");
    common_upsert(collection, payload).await;

    StatusCode::OK
}

async fn common_work_items(collection: Collection<WorkItem>) -> Vec<WorkItem> {
    let mut items: Vec<WorkItem> = Vec::new();
    match collection.find(doc! {}).await {
        Ok(docs) => {
            let mut cursor = docs;
            while let Some(workitem) = cursor.try_next().await.unwrap() {
                items.push(workitem);
            }
        }
        Err(e) => {
            tracing::error!("failed to query from modern {:?}", e);
        }
    }

    items
}

async fn all_work_items() -> (StatusCode, Json<Vec<WorkItem>>) {
    let modern_client = Client::with_uri_str("mongodb://standup:mongodb@standup-modern-0.standup-modern-svc.standup.svc.cluster.local:27017/admin?replicaSet=standup-modern&ssl=false") .await .unwrap();
    let modern_collection: Collection<WorkItem> =
        modern_client.database("standup").collection("work-items");
    let modern_items = common_work_items(modern_collection).await;

    let legacy_client = Client::with_uri_str("mongodb://standup:mongodb@standup-legacy-0.standup-legacy-svc.standup.svc.cluster.local:27017/admin?replicaSet=standup-legacy&ssl=false") .await .unwrap();
    let legacy_collection: Collection<WorkItem> =
        legacy_client.database("standup").collection("work-items");
    let legacy_items = common_work_items(legacy_collection).await;

    let mut items: Vec<WorkItem> = Vec::new();
    items.extend(modern_items);
    items.extend(legacy_items);

    tracing::info!("returning {} work items", items.len());
    (StatusCode::OK, Json(items))
}

async fn handle_404() -> StatusCode {
    StatusCode::NOT_FOUND
}

#[derive(Serialize)]
struct Health {
    up: bool,
}

async fn health() -> (StatusCode, Json<Health>) {
    // NOTE: if this system wasn't a joke, hard-coding credentials would be a Very Bad Idea
    // NOTE: I know I should use some sort of connection pool, but there is no time
    Client::with_uri_str("mongodb://standup:mongodb@mongodb-standup-0.mongodb-standup-svc.standup.svc.cluster.local:27017/admin?replicaSet=mongodb-standup&ssl=false")
        .await
        .unwrap();
    let body = Health { up: true };
    (StatusCode::OK, Json(body))
}
