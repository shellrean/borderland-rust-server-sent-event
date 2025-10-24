# 📡 Axum SSE Broadcast Server

This project is a lightweight real-time messaging server built with [Axum](https://docs.rs/axum/latest/axum/), using **Server-Sent Events (SSE)** and **topic-based broadcasting**. It allows clients to subscribe to specific topics and receive live messages pushed from the server.

---

## 🚀 Features

- 🔄 Real-time streaming via Server-Sent Events (SSE)
- 📨 Topic-based broadcasting using `tokio::broadcast`
- 🧵 Thread-safe shared state with `Arc<Mutex<...>>`
- ⚡ Built with async Rust using `tokio` and `axum`

---

## 📁 Project Structure

### `AppState`

```rust
struct AppState {
    topics: Arc<Mutex<HashMap<String, broadcast::Sender<BroadcastMessage>>>>,
}
