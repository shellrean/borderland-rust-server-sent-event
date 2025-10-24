use axum::{
    Json, Router,
    extract::{Query, State},
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
    topics: Arc<Mutex<HashMap<String, broadcast::Sender<BroadcastMessage>>>>,
}

async fn sse_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let topic_id = params.get("topic_id").cloned().unwrap_or_default();
    let tx = {
        let mut topics = state.topics.lock().await;
        topics
            .entry(topic_id)
            .or_insert_with(|| broadcast::channel::<BroadcastMessage>(100).0)
            .clone()
    };

    let rx = tx.subscribe();

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

async fn broadcast_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BroadcastMessage>,
) {
    let topics = state.topics.lock().await;
    if let Some(tx) = topics.get(&payload.topic) {
        let _ = tx.send(payload);
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        topics: Arc::new(Mutex::new(HashMap::new())),
    });

    let app = Router::new()
        .route("/sse", get(sse_handler))
        .route("/broadcast", post(broadcast_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
