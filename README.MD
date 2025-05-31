# 🚀 EmitHub - Real-time Broadcasting Microservice

**EmitHub** is a robust microservice for real-time communication built in Rust with Actix-web and ReDB. It enables creating isolated broadcasting channels where multiple clients can connect and receive instant broadcast messages.

---

## 📋 Table of Contents

- [🎯 Features](#-features)
- [🏗️ Architecture](#️-architecture)
- [📦 Installation](#-installation)
- [⚙️ Configuration](#️-configuration)
- [🚀 Quick Start](#-quick-start)
- [📡 API Reference](#-api-reference)
- [🔌 WebSocket](#-websocket)
- [💼 Use Cases](#-use-cases)
- [🧪 Practical Examples](#-practical-examples)
- [🐳 Deployment](#-deployment)
- [🔧 Development](#-development)

---

## 🎯 Features

### ✅ **Core Features**
- **Isolated Channels**: Each channel is independent with its own state
- **Emission Control**: Granular Start/Pause/Stop per channel
- **Real-time WebSocket**: Low-latency bidirectional connections
- **Optional Persistence**: Messages stored in ReDB when configured
- **Zero Dependencies**: Single binary with embedded database

### ✅ **Enterprise Ready**
- **Health Checks**: Service health monitoring
- **Structured Logging**: JSON-formatted logs for analysis
- **Configurable CORS**: Support for web applications
- **Rate Limiting**: Message speed control
- **Flexible Configuration**: Environment variables and TOML files

### ✅ **Developer Experience**
- **Integrated Documentation**: `/docs` endpoint with examples
- **Hot Config Reload**: Reload configuration without restart
- **Testing Ready**: APIs easy to test automatically
- **Self-Contained**: No external dependencies to run

---

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client App    │    │   Client App    │    │   Admin Panel   │
│   (WebSocket)   │    │   (WebSocket)   │    │   (REST API)    │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │      EmitHub Server       │
                    │   (Actix-web + ReDB)     │
                    └─────────────┬─────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │     Channel Manager       │
                    │  ┌─────────┬─────────┐   │
                    │  │Channel A│Channel B│   │
                    │  │ Active  │ Paused  │   │
                    │  └─────────┴─────────┘   │
                    └───────────────────────────┘
```

### **Main Components:**

- **API Layer**: REST endpoints for channel management
- **WebSocket Layer**: Real-time connections for clients
- **Channel Manager**: State logic and broadcasting
- **Persistence Layer**: ReDB for local storage
- **Configuration**: Flexible configuration system

---

## 📦 Installation

### **Prerequisites**
- Rust 1.70+
- Cargo
- (Optional) Docker for deployment

### **Method 1: From Source Code**

```bash
# Clone repository
git clone https://github.com/your-org/emit-hub.git
cd emit-hub

# Build
cargo build --release

# Run
./target/release/emit-hub
```

### **Method 2: Cargo Install**

```bash
# Install from crates.io
cargo install emit-hub

# Run
emit-hub
```

### **Method 3: Docker**

```bash
# Build image
docker build -t emit-hub .

# Run
docker run -p 8080:8080 emit-hub
```

---

## ⚙️ Configuration

### **Environment Variables**

```bash
# Server Configuration
export EMIT_HUB_HOST=0.0.0.0              # Host (default: 127.0.0.1)
export EMIT_HUB_PORT=3000                 # Port (default: 8080)
export EMIT_HUB_LOG_LEVEL=debug           # Log level (default: info)

# Database
export EMIT_HUB_DB_PATH=./data/hub.redb   # ReDB file (default: emit_hub.redb)

# Limits
export EMIT_HUB_MAX_CONNECTIONS=5000      # Max connections per channel (default: 1000)
export EMIT_HUB_MESSAGE_SIZE_LIMIT=2097152 # Max message size 2MB (default: 1MB)

# WebSocket
export EMIT_HUB_WS_TIMEOUT=60             # Connection timeout seconds (default: 30)
export EMIT_HUB_WS_PING_INTERVAL=45       # Ping interval seconds (default: 30)

# CORS
export EMIT_HUB_CORS_ORIGINS="*"          # Allowed origins (default: localhost)

# Persistence
export EMIT_HUB_PERSIST_MESSAGES=true     # Save messages (default: false)
export EMIT_HUB_MESSAGE_RETENTION_DAYS=90 # Retention days (default: 30)
export EMIT_HUB_AUTO_BACKUP=true          # Auto backup (default: false)
```

### **Configuration File (emit_hub.toml)**

```toml
host = "0.0.0.0"
port = 3000
db_path = "./data/emit_hub.redb"
max_connections_per_channel = 5000
message_size_limit = 2097152
log_level = "debug"

[cors]
allowed_origins = ["*"]
allowed_methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
allowed_headers = ["Content-Type", "Authorization", "Accept"]
max_age = 7200

[websocket]
connection_timeout = 60
max_continuation_size = 2097152
ping_interval = 45
pong_timeout = 15

[persistence]
persist_messages_default = true
max_messages_per_channel = 50000
message_retention_days = 90
auto_backup = true
backup_interval_hours = 12
```

### **Configuration Help**

```bash
# View all available variables
emit-hub --show-env-vars

# Generate example configuration file
emit-hub --generate-config > emit_hub.toml
```

---

## 🚀 Quick Start

### **1. Start the Server**

```bash
# Basic configuration
export EMIT_HUB_HOST=0.0.0.0
export EMIT_HUB_PORT=8080

# Run
cargo run
```

**Expected output:**
```
🚀 EmitHub - Real-time Broadcasting Service
   Version: 0.1.0
   Repository: https://github.com/your-org/emit-hub

2024-06-01T10:30:00.123Z  INFO emit_hub: 📡 Starting EmitHub on 0.0.0.0:8080
2024-06-01T10:30:00.124Z  INFO emit_hub: 💾 Database: emit_hub.redb
2024-06-01T10:30:00.125Z  INFO emit_hub: 🔌 Max connections per channel: 1000
2024-06-01T10:30:00.130Z  INFO emit_hub: ✅ Application state initialized successfully
```

### **2. Create a Channel**

```bash
curl -X POST http://localhost:8080/api/v1/channels \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Queue System",
    "description": "Channel for calling queue numbers in clinic",
    "settings": {
      "max_connections": 100,
      "allow_client_messages": true,
      "persist_messages": false,
      "rate_limit_per_minute": 30
    }
  }'
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Queue System",
  "description": "Channel for calling queue numbers in clinic",
  "status": "Created",
  "created_at": "2024-06-01T10:35:00Z",
  "updated_at": "2024-06-01T10:35:00Z",
  "settings": {
    "max_connections": 100,
    "allow_client_messages": true,
    "persist_messages": false,
    "rate_limit_per_minute": 30
  }
}
```

### **3. Activate the Channel**

```bash
# Use the UUID from the previous step
curl -X PUT http://localhost:8080/api/v1/channels/550e8400-e29b-41d4-a716-446655440000/start
```

### **4. Connect Clients**

```javascript
// JavaScript client
const channelId = '550e8400-e29b-41d4-a716-446655440000';
const ws = new WebSocket(`ws://localhost:8080/api/v1/channels/${channelId}/ws`);

ws.onopen = function() {
    console.log('🟢 Connected to channel');
};

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    console.log('📨 Message received:', data);
    
    if (data.status === 'broadcast') {
        console.log('🎯 Queue called:', data.message);
        // Display on screen, play sound, etc.
    }
};

ws.onerror = function(error) {
    console.error('❌ Connection error:', error);
};

// Send message (if allowed)
ws.send('Client connected from waiting room');
```

### **5. Broadcast Queue Numbers**

```bash
# Call queue A004
curl -X POST http://localhost:8080/api/v1/channels/550e8400-e29b-41d4-a716-446655440000/broadcast \
  -H "Content-Type: application/json" \
  -d '{
    "content": "A004",
    "message_type": "Broadcast"
  }'
```

**Clients receive:**
```json
{
  "status": "broadcast",
  "message": "A004",
  "channel_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-06-01T10:40:00Z",
  "data": null
}
```

---

## 📡 API Reference

### **Base URL**: `http://localhost:8080/api/v1`

### **🏢 Channel Management**

#### **Create Channel**
```http
POST /channels
Content-Type: application/json

{
  "name": "Channel Name",
  "description": "Optional description",
  "settings": {
    "max_connections": 1000,
    "allow_client_messages": true,
    "persist_messages": false,
    "rate_limit_per_minute": 60
  }
}
```

#### **List Channels**
```http
GET /channels
```

#### **Get Channel**
```http
GET /channels/{channel_id}
```

#### **Start Broadcasting**
```http
PUT /channels/{channel_id}/start
```

#### **Pause Broadcasting**
```http
PUT /channels/{channel_id}/pause
```

#### **Stop Broadcasting**
```http
PUT /channels/{channel_id}/stop
```

#### **Broadcast Message**
```http
POST /channels/{channel_id}/broadcast
Content-Type: application/json

{
  "content": "Message to send",
  "message_type": "Broadcast"
}
```

### **❤️ Health & Monitoring**

#### **Health Check**
```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "service": "emit-hub",
  "version": "0.1.0",
  "timestamp": "2024-06-01T10:30:00Z"
}
```

#### **Readiness Check**
```http
GET /ready
```

**Response:**
```json
{
  "status": "ready",
  "checks": {
    "database": "ok",
    "memory": "ok"
  }
}
```

### **📄 Information and Documentation**

#### **Service Information**
```http
GET /
```

#### **Interactive Documentation**
```http
GET /docs
```

---

## 🔌 WebSocket

### **Endpoint**: `ws://localhost:8080/api/v1/channels/{channel_id}/ws`

### **Connection States**

#### **✅ Successful Connection**
```json
{
  "status": "connected",
  "message": "Connected to channel: Queue System",
  "channel_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-06-01T10:35:00Z",
  "data": {
    "channel": { /* channel info */ },
    "connection_id": "123e4567-e89b-12d3-a456-426614174000"
  }
}
```

#### **❌ Channel Not Active**
```json
{
  "status": "denied",
  "message": "Channel Queue System is not active",
  "channel_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-06-01T10:35:00Z",
  "data": null
}
```

### **Message Types**

#### **Server Broadcast**
```json
{
  "status": "broadcast",
  "message": "A004",
  "channel_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-06-01T10:40:00Z",
  "data": null
}
```

#### **Client Message**
```json
{
  "status": "client_message",
  "message": "Client message in Queue System: Hello",
  "channel_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-06-01T10:41:00Z",
  "data": {
    "original_message": "Hello",
    "sender": "client"
  }
}
```

### **WebSocket Error Handling**

```javascript
ws.onerror = function(error) {
    console.error('WebSocket Error:', error);
};

ws.onclose = function(event) {
    if (event.wasClean) {
        console.log('Connection closed cleanly');
    } else {
        console.log('Connection lost, retrying...');
        // Implement automatic reconnection
        setTimeout(() => {
            connectWebSocket();
        }, 5000);
    }
};
```

---

## 💼 Use Cases

### **🏥 Medical Queue System**

**Scenario**: Clinic with multiple consultation rooms and waiting room with displays.

```bash
# 1. Create channel per consultation room
curl -X POST http://localhost:8080/api/v1/channels \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Dr. Garcia Office",
    "settings": {
      "max_connections": 50,
      "allow_client_messages": false,
      "persist_messages": true
    }
  }'

# 2. Activate channel
curl -X PUT http://localhost:8080/api/v1/channels/{channel_id}/start

# 3. Displays connect automatically
# 4. Receptionist calls queue numbers
curl -X POST http://localhost:8080/api/v1/channels/{channel_id}/broadcast \
  -d '{"content": "A004", "message_type": "Broadcast"}'
```

### **🏢 Corporate Alerts**

**Scenario**: Company with critical alerts per department.

```bash
# Channel per department
curl -X POST http://localhost:8080/api/v1/channels \
  -d '{
    "name": "IT Alerts",
    "settings": {
      "max_connections": 200,
      "allow_client_messages": true,
      "rate_limit_per_minute": 10
    }
  }'

# Monitoring system sends alerts
curl -X POST http://localhost:8080/api/v1/channels/{channel_id}/broadcast \
  -d '{"content": "🚨 Main server down", "message_type": "Broadcast"}'
```

### **📊 Real-time Dashboard**

**Scenario**: Sales metrics updating live.

```javascript
// Dashboard client
const ws = new WebSocket(`ws://localhost:8080/api/v1/channels/${metricsChannelId}/ws`);

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    
    if (data.status === 'broadcast') {
        const metrics = JSON.parse(data.message);
        updateDashboard(metrics);
    }
};

// Sales system sends updates every minute
setInterval(() => {
    const metrics = {
        sales_today: 15420,
        active_users: 89,
        conversion_rate: 3.2
    };
    
    fetch(`/api/v1/channels/${channelId}/broadcast`, {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({
            content: JSON.stringify(metrics),
            message_type: 'Broadcast'
        })
    });
}, 60000);
```

### **🎮 Game Notifications**

**Scenario**: Notify real-time events to players.

```bash
# Channel for game events
curl -X POST http://localhost:8080/api/v1/channels \
  -d '{
    "name": "Game Events",
    "settings": {
      "max_connections": 10000,
      "allow_client_messages": false,
      "persist_messages": false
    }
  }'

# Event: new player joins
curl -X POST http://localhost:8080/api/v1/channels/{channel_id}/broadcast \
  -d '{"content": "🎉 Player MasterGamer joined the server!", "message_type": "Broadcast"}'
```

---

## 🧪 Practical Examples

### **Dart Client (Flutter)**

```dart
import 'dart:convert';
import 'package:web_socket_channel/web_socket_channel.dart';

class EmitHubClient {
  final String baseUrl;
  final String channelId;
  WebSocketChannel? _channel;
  
  EmitHubClient(this.baseUrl, this.channelId);
  
  Future<void> connect() async {
    try {
      final wsUrl = baseUrl.replaceFirst('http', 'ws');
      _channel = WebSocketChannel.connect(
        Uri.parse('$wsUrl/api/v1/channels/$channelId/ws')
      );
      
      _channel!.stream.listen(
        (message) {
          final data = jsonDecode(message);
          handleMessage(data);
        },
        onError: (error) {
          print('Error: $error');
        },
      );
    } catch (e) {
      print('Connection failed: $e');
    }
  }
  
  void handleMessage(Map<String, dynamic> data) {
    switch (data['status']) {
      case 'broadcast':
        print('🎯 Queue called: ${data['message']}');
        showQueueNotification(data['message']);
        break;
      case 'connected':
        print('✅ Connected to channel');
        break;
      default:
        print('📨 Message: ${data['message']}');
    }
  }
  
  void showQueueNotification(String queue) {
    // Show notification in Flutter
    // Play sound
    // Update UI
  }
  
  void disconnect() {
    _channel?.sink.close();
  }
}
```

### **React Client (JavaScript)**

```javascript
import React, { useState, useEffect } from 'react';

const EmitHubClient = ({ channelId }) => {
  const [messages, setMessages] = useState([]);
  const [connectionStatus, setConnectionStatus] = useState('disconnected');
  const [ws, setWs] = useState(null);

  useEffect(() => {
    connectWebSocket();
    return () => {
      if (ws) {
        ws.close();
      }
    };
  }, [channelId]);

  const connectWebSocket = () => {
    const websocket = new WebSocket(
      `ws://localhost:8080/api/v1/channels/${channelId}/ws`
    );

    websocket.onopen = () => {
      setConnectionStatus('connected');
      console.log('🟢 Connected to EmitHub');
    };

    websocket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      
      setMessages(prev => [...prev, data]);
      
      if (data.status === 'broadcast') {
        // Show toast notification
        showNotification(data.message);
        
        // Play sound
        playNotificationSound();
      }
    };

    websocket.onclose = () => {
      setConnectionStatus('disconnected');
      console.log('🔴 Disconnected from EmitHub');
      
      // Auto-reconnect
      setTimeout(connectWebSocket, 5000);
    };

    setWs(websocket);
  };

  const showNotification = (message) => {
    // Use notification library like react-toast
    toast.success(`🎯 Queue: ${message}`, {
      position: 'top-center',
      autoClose: 10000,
    });
  };

  const playNotificationSound = () => {
    const audio = new Audio('/notification.mp3');
    audio.play().catch(e => console.log('Error playing sound:', e));
  };

  return (
    <div className="emit-hub-client">
      <div className={`status ${connectionStatus}`}>
        Status: {connectionStatus === 'connected' ? '🟢 Connected' : '🔴 Disconnected'}
      </div>
      
      <div className="messages">
        {messages.map((msg, index) => (
          <div key={index} className={`message ${msg.status}`}>
            <span className="time">
              {new Date(msg.timestamp).toLocaleTimeString()}
            </span>
            <span className="content">{msg.message}</span>
          </div>
        ))}
      </div>
    </div>
  );
};

export default EmitHubClient;
```

### **Python Client**

```python
import asyncio
import websockets
import json
import logging

class EmitHubClient:
    def __init__(self, base_url: str, channel_id: str):
        self.base_url = base_url.replace('http', 'ws')
        self.channel_id = channel_id
        self.websocket = None
        
    async def connect(self):
        uri = f"{self.base_url}/api/v1/channels/{self.channel_id}/ws"
        
        try:
            self.websocket = await websockets.connect(uri)
            logging.info(f"🟢 Connected to EmitHub: {uri}")
            
            await self.listen_messages()
            
        except Exception as e:
            logging.error(f"❌ Connection error: {e}")
            
    async def listen_messages(self):
        async for message in self.websocket:
            try:
                data = json.loads(message)
                await self.handle_message(data)
            except json.JSONDecodeError:
                logging.error(f"Error parsing message: {message}")
                
    async def handle_message(self, data):
        status = data.get('status')
        message = data.get('message')
        
        if status == 'broadcast':
            print(f"🎯 Queue called: {message}")
            # Here you can add specific logic
            # like sending email, updating database, etc.
            
        elif status == 'connected':
            print(f"✅ Connected to channel: {data.get('channel_id')}")
            
        else:
            print(f"📨 Message: {message}")
            
    async def disconnect(self):
        if self.websocket:
            await self.websocket.close()

# Usage
async def main():
    client = EmitHubClient(
        base_url="http://localhost:8080",
        channel_id="550e8400-e29b-41d4-a716-446655440000"
    )
    
    await client.connect()

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    asyncio.run(main())
```

### **Admin Panel (Channel Management)**

```javascript
// Functions to manage channels from admin panel

class EmitHubAdmin {
  constructor(baseUrl) {
    this.baseUrl = baseUrl;
  }

  async createChannel(channelData) {
    const response = await fetch(`${this.baseUrl}/api/v1/channels`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(channelData)
    });
    
    return await response.json();
  }

  async listChannels() {
    const response = await fetch(`${this.baseUrl}/api/v1/channels`);
    return await response.json();
  }

  async startChannel(channelId) {
    const response = await fetch(
      `${this.baseUrl}/api/v1/channels/${channelId}/start`,
      { method: 'PUT' }
    );
    return await response.json();
  }

  async pauseChannel(channelId) {
    const response = await fetch(
      `${this.baseUrl}/api/v1/channels/${channelId}/pause`,
      { method: 'PUT' }
    );
    return await response.json();
  }

  async stopChannel(channelId) {
    const response = await fetch(
      `${this.baseUrl}/api/v1/channels/${channelId}/stop`,
      { method: 'PUT' }
    );
    return await response.json();
  }

  async broadcastMessage(channelId, content, messageType = 'Broadcast') {
    const response = await fetch(
      `${this.baseUrl}/api/v1/channels/${channelId}/broadcast`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content, message_type: messageType })
      }
    );
    return await response.json();
  }
}

// Example usage in admin panel
const admin = new EmitHubAdmin('http://localhost:8080');

// Create channel for medical queue
const channel = await admin.createChannel({
  name: 'Cardiologist Office',
  description: 'Channel for cardiology queue',
  settings: {
    max_connections: 100,
    allow_client_messages: false,
    persist_messages: true,
    rate_limit_per_minute: 30
  }
});

// Activate the channel
await admin.startChannel(channel.id);

// Send queue number
await admin.broadcastMessage(channel.id, 'A015');
```

---

## 🐳 Deployment

### **Docker**

#### **Dockerfile**
```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/emit-hub /usr/local/bin/emit-hub

EXPOSE 8080

CMD ["emit-hub"]
```

#### **docker-compose.yml**
```yaml
version: '3.8'

services:
  emit-hub:
    build: .
    ports:
      - "8080:8080"
    environment:
      - EMIT_HUB_HOST=0.0.0.0
      - EMIT_HUB_PORT=8080
      - EMIT_HUB_LOG_LEVEL=info
      - EMIT_HUB_MAX_CONNECTIONS=5000
      - EMIT_HUB_PERSIST_MESSAGES=true
    volumes:
      - ./data:/app/data
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/v1/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

### **Kubernetes**

#### **deployment.yaml**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: emit-hub
spec:
  replicas: 3
  selector:
    matchLabels:
      app: emit-hub
  template:
    metadata:
      labels:
        app: emit-hub
    spec:
      containers:
      - name: emit-hub
        image: emit-hub:latest
        ports:
        - containerPort: 8080
        env:
        - name: EMIT_HUB_HOST
          value: "0.0.0.0"
        - name: EMIT_HUB_PORT
          value: "8080"
        - name: EMIT_HUB_LOG_LEVEL
          value: "info"
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/v1/ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
```

#### **service.yaml**
```yaml
apiVersion: v1
kind: Service
metadata:
  name: emit-hub-service
spec:
  selector:
    app: emit-hub
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
  type: LoadBalancer
```

### **Production Deployment**

#### **Systemd Service**
```ini
# /etc/systemd/system/emit-hub.service
[Unit]
Description=EmitHub Real-time Broadcasting Service
After=network.target

[Service]
Type=simple
User=emit-hub
WorkingDirectory=/opt/emit-hub
ExecStart=/opt/emit-hub/emit-hub
Restart=always
RestartSec=5
Environment=EMIT_HUB_HOST=0.0.0.0
Environment=EMIT_HUB_PORT=8080
Environment=EMIT_HUB_LOG_LEVEL=info
Environment=EMIT_HUB_DB_PATH=/var/lib/emit-hub/emit_hub.redb

[Install]
WantedBy=multi-user.target
```

#### **Nginx Reverse Proxy**
```nginx
upstream emit_hub {
    server 127.0.0.1:8080;
}

server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://emit_hub;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
        proxy_read_timeout 86400;
    }
}
```

---

## 🔧 Development

### **Project Structure**

```
emit-hub/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── state.rs             # Application state
│   ├── models/
│   │   ├── mod.rs
│   │   ├── channel.rs       # Channel data models
│   │   └── message.rs       # Message data models
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── channels.rs      # Channel REST endpoints
│   │   ├── websocket.rs     # WebSocket handlers
│   │   └── health.rs        # Health check endpoints
│   └── utils/
│       ├── mod.rs
│       └── logging.rs       # Logging utilities
├── tests/
│   ├── integration_tests.rs
│   └── websocket_tests.rs
├── docs/
│   ├── api.md
│   └── deployment.md
├── docker/
│   ├── Dockerfile
│   └── docker-compose.yml
├── scripts/
│   ├── setup.sh
│   └── deploy.sh
├── Cargo.toml
├── Cargo.lock
└── README.md
```

### **Development Setup**

```bash
# Clone and setup
git clone https://github.com/your-org/emit-hub.git
cd emit-hub

# Install development dependencies
cargo install cargo-watch cargo-tarpaulin

# Setup git hooks
cp scripts/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit

# Run in development mode with auto-reload
cargo watch -x run

# Run tests
cargo test

# Generate test coverage
cargo tarpaulin --out Html
```

### **Testing**

#### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_create_channel() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .service(create_channel)
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/v1/channels")
            .set_json(&channel_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

#### **Integration Tests**
```rust
// tests/integration_tests.rs
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::test]
async fn test_websocket_connection() {
    let server = start_test_server().await;
    
    let (ws_stream, _) = connect_async("ws://localhost:8080/api/v1/channels/test/ws")
        .await
        .expect("Failed to connect");

    // Test connection and message handling
    assert!(ws_stream.is_ok());
}
```

#### **Load Testing**
```bash
# Using Artillery
npm install -g artillery

# Test WebSocket connections
artillery run tests/load/websocket.yml

# Test REST API
artillery run tests/load/rest-api.yml
```

### **Performance Monitoring**

#### **Metrics Collection**
```rust
use prometheus::{Encoder, TextEncoder, Counter, Histogram, register_counter, register_histogram};

lazy_static! {
    static ref HTTP_REQUESTS: Counter = register_counter!(
        "http_requests_total", "Total HTTP requests"
    ).unwrap();
    
    static ref WEBSOCKET_CONNECTIONS: Counter = register_counter!(
        "websocket_connections_total", "Total WebSocket connections"
    ).unwrap();
    
    static ref MESSAGE_DURATION: Histogram = register_histogram!(
        "message_broadcast_duration_seconds", "Message broadcast duration"
    ).unwrap();
}

#[get("/metrics")]
async fn metrics() -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer)
}
```

#### **Health Monitoring**
```yaml
# docker-compose.monitoring.yml
version: '3.8'

services:
  emit-hub:
    # ... your service config

  prometheus:
    image: prom/prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana
    ports:
      - "3000:3000"
    volumes:
      - ./monitoring/grafana:/var/lib/grafana
```

### **Contributing**

#### **Code Style**
```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Check documentation
cargo doc --no-deps --open
```

#### **Commit Convention**
```bash
# Format: type(scope): description
feat(channels): add channel persistence support
fix(websocket): handle connection timeout properly
docs(readme): update installation instructions
perf(broadcast): optimize message delivery
```

#### **Pull Request Process**
1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Make changes and add tests
4. Ensure all tests pass: `cargo test`
5. Format code: `cargo fmt`
6. Submit pull request

### **Debugging**

#### **Enable Debug Logging**
```bash
RUST_LOG=debug cargo run
RUST_LOG=emit_hub=trace cargo run
```

#### **Profiling**
```bash
# Install profiling tools
cargo install flamegraph

# Generate flame graph
cargo flamegraph --bin emit-hub

# Memory profiling with valgrind
cargo build --release
valgrind --tool=massif target/release/emit-hub
```

#### **Database Inspection**
```rust
// src/utils/db_tools.rs
pub async fn inspect_database(db_path: &str) -> anyhow::Result<()> {
    let db = Database::open(db_path)?;
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(CHANNELS_TABLE)?;
    
    println!("Database contents:");
    for result in table.iter()? {
        let (key, value) = result?;
        let channel: Channel = serde_json::from_slice(value.value())?;
        println!("Channel {}: {:?}", key.value(), channel);
    }
    
    Ok(())
}
```

---

## 🚀 Production Checklist

### **Pre-deployment**
- [ ] All tests passing
- [ ] Performance benchmarks met
- [ ] Security audit completed
- [ ] Documentation updated
- [ ] Monitoring configured
- [ ] Backup strategy implemented

### **Security**
- [ ] HTTPS/WSS in production
- [ ] Rate limiting configured
- [ ] CORS properly set
- [ ] Input validation enabled
- [ ] Error messages sanitized
- [ ] Dependencies updated

### **Monitoring**
- [ ] Health checks enabled
- [ ] Metrics collection active
- [ ] Log aggregation configured
- [ ] Alerting rules set
- [ ] Dashboard created

### **Backup & Recovery**
- [ ] Database backup automated
- [ ] Recovery procedures tested
- [ ] Disaster recovery plan documented
- [ ] RTO/RPO defined

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### **Areas for Contribution**
- 🐛 **Bug Reports**: Help us identify and fix issues
- ✨ **Feature Requests**: Suggest new functionality
- 📚 **Documentation**: Improve guides and examples
- 🧪 **Testing**: Add test cases and scenarios
- 🎨 **Client Libraries**: Build clients for different languages
- 🔧 **Tools**: Create utilities and integrations

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Actix-web** team for the excellent web framework
- **ReDB** developers for the embedded database
- **Rust** community for the amazing ecosystem
- All contributors and users of EmitHub

---
- 🐛 **Issues**: [GitHub Issues](https://github.com/JhonaCodes/emit-hub/issues)
---

**Made with ❤️ by Jhonacodes**