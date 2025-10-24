# Rust SSE Broadcast System

This project demonstrates how to build a **real-time broadcast system** in **Rust** using **Server-Sent Events (SSE)**.
It allows messages sent via **REST API endpoints** to be **broadcasted to all connected SSE clients**.

---

## 🚀 Overview

The system is designed to handle real-time updates efficiently without requiring WebSockets.
Each client connects to an SSE endpoint to receive events, while a REST API is used to publish messages to all listeners.

This architecture is ideal for use cases such as:
- Real-time dashboards
- Notification systems
- Live logs or monitoring tools
- Chat message broadcasting

---

## 🧩 How It Works

1. **Clients connect** to an SSE endpoint:
   - Each client subscribes to a stream of events using a unique `client_id`.
2. **Messages are published** through a REST endpoint:
   - The REST endpoint receives a message payload and sends it to all subscribers using a broadcast channel.
3. **SSE clients receive** the broadcasted messages in real time.

---

## ⚙️ Technology Stack

- **Rust**
- **Axum** – Web framework for routing and request handling
- **tokio** – Async runtime for concurrency
- **Server-Sent Events (SSE)** – Real-time one-way event streaming
- **Broadcast channel** (`tokio::sync::broadcast`) – Message distribution mechanism

---

## 📡 Example Flow

1. Start the server.
2. Open multiple browser tabs or clients connected to `/sse?client_id=<id>`.
3. Use a REST client (like Postman or curl) to send a message to `/send`.
4. All connected SSE clients instantly receive the message.

---

## 🧠 Key Features

- Supports multiple clients simultaneously.
- Uses a broadcast mechanism for efficient event distribution.
- Lightweight and simple — no need for WebSocket complexity.
- Thread-safe shared state using `Arc<Mutex<...>>`.

---

## 🧪 Example Endpoints

### SSE Subscription
```bash
GET /sse?client_id=123
