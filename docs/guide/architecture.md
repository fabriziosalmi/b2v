# Architecture Overview

## Frontend
- Built with TypeScript and React
- Features:
  - Real-time data updates
  - State management using Redux
  - Responsive design for all devices

## Backend
- Built with Rust and WebAssembly (WASM)
- Features:
  - High-performance data processing
  - Secure API endpoints
  - Integration with frontend via WebSockets

## Communication
- Frontend and backend communicate through a WebSocket connection
- Data is serialized using JSON format

## Example Usage

### Frontend Example: Connecting to WebSocket
```ts
import { useEffect, useState } from 'react';

const WebSocketExample = () => {
  const [message, setMessage] = useState('');

  useEffect(() => {
    const ws = new WebSocket('wss://example.com/socket');

    ws.onmessage = (event) => {
      setMessage(event.data);
    };

    return () => ws.close();
  }, []);

  return (
    <div>
      <h2>WebSocket Message</h2>
      <p>{message}</p>
    </div>
  );
};

export default WebSocketExample;
```

### Backend Example: Handling WebSocket Connection
```rust
use tungstenite::WebSocket;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
  let (tx, rx) = mpsc::channel(32);

  // WebSocket server setup
  let ws_server = tungstenite::WebSocketServer::new(|conn| {
    let (sender, receiver) = mpsc::channel(32);

    // Handle incoming messages
    conn.on_message(move |msg| {
      if let Ok(text) = msg.into_text() {
        sender.send(text).unwrap();
      }
    });

    // Forward messages to the main thread
    tokio::spawn(async move {
      while let Some(text) = receiver.recv().await {
        tx.send(text).unwrap();
      }
    });
  });

  // Start the server
  ws_server.start("127.0.0.1:8080").await.unwrap();
}
```
