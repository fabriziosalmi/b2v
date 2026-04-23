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