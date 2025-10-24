use axum::{
    Json, Router,
    extract::State,
    response::{Sse, sse::Event},
    routing::{get, post},
};
use futures_util::stream::Stream;
use serde::Deserialize;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{Mutex, broadcast};
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<BroadcastMessage>,
    topics: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
}

async fn sse_handler(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx).map(|msg| match msg {
        Ok(text) => Ok(Event::default().data(text.message)),
        Err(_) => Ok(Event::default().data("<error>")),
    });
    Sse::new(stream)
}

#[derive(Deserialize, Clone)]
struct BroadcastMessage {
    message: String,
    topic: String,
}

#[derive(Deserialize)]
struct SubscribeRequest {
    client_id: String,
    topic: String,
}

async fn broadcast_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BroadcastMessage>,
) {
    let _ = state.tx.send(payload);
}

async fn subscribe_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SubscribeRequest>,
) {
    state
        .topics
        .lock()
        .await
        .entry(payload.client_id)
        .or_default()
        .insert(payload.topic, "subscribed".to_string());
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<BroadcastMessage>(100);
    let state = Arc::new(AppState {
        tx,
        topics: Arc::new(Mutex::new(HashMap::new())),
    });

    let app = Router::new()
        .route("/sse", get(sse_handler))
        .route("/broadcast", post(broadcast_handler))
        .route("/subscribe", post(subscribe_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
