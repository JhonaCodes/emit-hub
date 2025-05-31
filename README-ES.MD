# 🚀 EmitHub - Real-time Broadcasting Microservice

**EmitHub** es un microservicio robusto para comunicación en tiempo real construido en Rust con Actix-web y ReDB. Permite crear canales de emisión aislados donde múltiples clientes pueden conectarse y recibir mensajes broadcast instantáneamente.

---

## 📋 Tabla de Contenidos

- [🎯 Características](#-características)
- [🏗️ Arquitectura](#️-arquitectura)
- [📦 Instalación](#-instalación)
- [⚙️ Configuración](#️-configuración)
- [🚀 Inicio Rápido](#-inicio-rápido)
- [📡 API Reference](#-api-reference)
- [🔌 WebSocket](#-websocket)
- [💼 Casos de Uso](#-casos-de-uso)
- [🧪 Ejemplos Prácticos](#-ejemplos-prácticos)
- [🐳 Deploy](#-deploy)
- [🔧 Desarrollo](#-desarrollo)

---

## 🎯 Características

### ✅ **Core Features**
- **Canales Aislados**: Cada canal es independiente con su propio estado
- **Control de Emisión**: Start/Pause/Stop granular por canal
- **WebSocket Real-time**: Conexiones bidireccionales de baja latencia
- **Persistencia Opcional**: Mensajes guardados en ReDB si se configura
- **Zero Dependencies**: Una sola binaria con base de datos embebida

### ✅ **Enterprise Ready**
- **Health Checks**: Monitoreo de salud del servicio
- **Structured Logging**: Logs con formato JSON para análisis
- **CORS Configurable**: Soporte para aplicaciones web
- **Rate Limiting**: Control de velocidad de mensajes
- **Configuración Flexible**: Variables de entorno y archivos TOML

### ✅ **Developer Experience**
- **Documentación Integrada**: `/docs` endpoint con ejemplos
- **Hot Reload Config**: Recarga configuración sin reiniciar
- **Testing Ready**: APIs fáciles de testear automáticamente
- **Self-Contained**: Sin dependencias externas para funcionar

---

## 🏗️ Arquitectura

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

### **Componentes Principales:**

- **API Layer**: REST endpoints para gestión de canales
- **WebSocket Layer**: Conexiones en tiempo real para clientes
- **Channel Manager**: Lógica de estado y broadcasting
- **Persistence Layer**: ReDB para almacenamiento local
- **Configuration**: Sistema flexible de configuración

---

## 📦 Instalación

### **Prerrequisitos**
- Rust 1.70+
- Cargo
- (Opcional) Docker para deployment

### **Método 1: Desde Código Fuente**

```bash
# Clonar repositorio
git clone https://github.com/your-org/emit-hub.git
cd emit-hub

# Construir
cargo build --release

# Ejecutar
./target/release/emit-hub
```

### **Método 2: Cargo Install**

```bash
# Instalar desde crates.io
cargo install emit-hub

# Ejecutar
emit-hub
```

### **Método 3: Docker**

```bash
# Construir imagen
docker build -t emit-hub .

# Ejecutar
docker run -p 8080:8080 emit-hub
```

---

## ⚙️ Configuración

### **Variables de Entorno**

```bash
# Configuración del Servidor
export EMIT_HUB_HOST=0.0.0.0              # Host (default: 127.0.0.1)
export EMIT_HUB_PORT=3000                 # Puerto (default: 8080)
export EMIT_HUB_LOG_LEVEL=debug           # Nivel de log (default: info)

# Base de Datos
export EMIT_HUB_DB_PATH=./data/hub.redb   # Archivo ReDB (default: emit_hub.redb)

# Límites
export EMIT_HUB_MAX_CONNECTIONS=5000      # Max conexiones por canal (default: 1000)
export EMIT_HUB_MESSAGE_SIZE_LIMIT=2097152 # Max tamaño mensaje 2MB (default: 1MB)

# WebSocket
export EMIT_HUB_WS_TIMEOUT=60             # Timeout conexión segundos (default: 30)
export EMIT_HUB_WS_PING_INTERVAL=45       # Intervalo ping segundos (default: 30)

# CORS
export EMIT_HUB_CORS_ORIGINS="*"          # Orígenes permitidos (default: localhost)

# Persistencia
export EMIT_HUB_PERSIST_MESSAGES=true     # Guardar mensajes (default: false)
export EMIT_HUB_MESSAGE_RETENTION_DAYS=90 # Días retención (default: 30)
export EMIT_HUB_AUTO_BACKUP=true          # Backup automático (default: false)
```

### **Archivo de Configuración (emit_hub.toml)**

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

### **Ayuda de Configuración**

```bash
# Ver todas las variables disponibles
emit-hub --show-env-vars

# Generar archivo de configuración de ejemplo
emit-hub --generate-config > emit_hub.toml
```

---

## 🚀 Inicio Rápido

### **1. Iniciar el Servidor**

```bash
# Configuración básica
export EMIT_HUB_HOST=0.0.0.0
export EMIT_HUB_PORT=8080

# Ejecutar
cargo run
```

**Output esperado:**
```
🚀 EmitHub - Real-time Broadcasting Service
   Version: 0.1.0
   Repository: https://github.com/your-org/emit-hub

2024-06-01T10:30:00.123Z  INFO emit_hub: 📡 Starting EmitHub on 0.0.0.0:8080
2024-06-01T10:30:00.124Z  INFO emit_hub: 💾 Database: emit_hub.redb
2024-06-01T10:30:00.125Z  INFO emit_hub: 🔌 Max connections per channel: 1000
2024-06-01T10:30:00.130Z  INFO emit_hub: ✅ Application state initialized successfully
```

### **2. Crear un Canal**

```bash
curl -X POST http://localhost:8080/api/v1/channels \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Sistema de Turnos",
    "description": "Canal para llamar turnos en clínica",
    "settings": {
      "max_connections": 100,
      "allow_client_messages": true,
      "persist_messages": false,
      "rate_limit_per_minute": 30
    }
  }'
```

**Respuesta:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Sistema de Turnos",
  "description": "Canal para llamar turnos en clínica",
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

### **3. Activar el Canal**

```bash
# Usar el UUID del paso anterior
curl -X PUT http://localhost:8080/api/v1/channels/550e8400-e29b-41d4-a716-446655440000/start
```

### **4. Conectar Clientes**

```javascript
// Cliente JavaScript
const channelId = '550e8400-e29b-41d4-a716-446655440000';
const ws = new WebSocket(`ws://localhost:8080/api/v1/channels/${channelId}/ws`);

ws.onopen = function() {
    console.log('🟢 Conectado al canal');
};

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    console.log('📨 Mensaje recibido:', data);
    
    if (data.status === 'broadcast') {
        console.log('🎯 Turno llamado:', data.message);
        // Mostrar en pantalla, reproducir sonido, etc.
    }
};

ws.onerror = function(error) {
    console.error('❌ Error de conexión:', error);
};

// Enviar mensaje (si está permitido)
ws.send('Cliente conectado desde sala de espera');
```

### **5. Emitir Turnos**

```bash
# Llamar turno A004
curl -X POST http://localhost:8080/api/v1/channels/550e8400-e29b-41d4-a716-446655440000/broadcast \
  -H "Content-Type: application/json" \
  -d '{
    "content": "A004",
    "message_type": "Broadcast"
  }'
```

**Los clientes reciben:**
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

#### **Crear Canal**
```http
POST /channels
Content-Type: application/json

{
  "name": "Nombre del Canal",
  "description": "Descripción opcional",
  "settings": {
    "max_connections": 1000,
    "allow_client_messages": true,
    "persist_messages": false,
    "rate_limit_per_minute": 60
  }
}
```

#### **Listar Canales**
```http
GET /channels
```

#### **Obtener Canal**
```http
GET /channels/{channel_id}
```

#### **Iniciar Emisión**
```http
PUT /channels/{channel_id}/start
```

#### **Pausar Emisión**
```http
PUT /channels/{channel_id}/pause
```

#### **Detener Emisión**
```http
PUT /channels/{channel_id}/stop
```

#### **Emitir Mensaje**
```http
POST /channels/{channel_id}/broadcast
Content-Type: application/json

{
  "content": "Mensaje a enviar",
  "message_type": "Broadcast"
}
```

### **❤️ Health & Monitoring**

#### **Health Check**
```http
GET /health
```

**Respuesta:**
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

**Respuesta:**
```json
{
  "status": "ready",
  "checks": {
    "database": "ok",
    "memory": "ok"
  }
}
```

### **📄 Información y Documentación**

#### **Información del Servicio**
```http
GET /
```

#### **Documentación Interactiva**
```http
GET /docs
```

---

## 🔌 WebSocket

### **Endpoint**: `ws://localhost:8080/api/v1/channels/{channel_id}/ws`

### **Estados de Conexión**

#### **✅ Conexión Exitosa**
```json
{
  "status": "connected",
  "message": "Connected to channel: Sistema de Turnos",
  "channel_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-06-01T10:35:00Z",
  "data": {
    "channel": { /* info del canal */ },
    "connection_id": "123e4567-e89b-12d3-a456-426614174000"
  }
}
```

#### **❌ Canal No Activo**
```json
{
  "status": "denied",
  "message": "Channel Sistema de Turnos is not active",
  "channel_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-06-01T10:35:00Z",
  "data": null
}
```

### **Tipos de Mensajes**

#### **Broadcast del Servidor**
```json
{
  "status": "broadcast",
  "message": "A004",
  "channel_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-06-01T10:40:00Z",
  "data": null
}
```

#### **Mensaje de Cliente**
```json
{
  "status": "client_message",
  "message": "Client message in Sistema de Turnos: Hola",
  "channel_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-06-01T10:41:00Z",
  "data": {
    "original_message": "Hola",
    "sender": "client"
  }
}
```

### **Manejo de Errores WebSocket**

```javascript
ws.onerror = function(error) {
    console.error('Error WebSocket:', error);
};

ws.onclose = function(event) {
    if (event.wasClean) {
        console.log('Conexión cerrada limpiamente');
    } else {
        console.log('Conexión perdida, reintentando...');
        // Implementar reconexión automática
        setTimeout(() => {
            connectWebSocket();
        }, 5000);
    }
};
```

---

## 💼 Casos de Uso

### **🏥 Sistema de Turnos Médicos**

**Escenario**: Clínica con múltiples consultorios y sala de espera con pantallas.

```bash
# 1. Crear canal por consultorio
curl -X POST http://localhost:8080/api/v1/channels \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Consultorio Dr. García",
    "settings": {
      "max_connections": 50,
      "allow_client_messages": false,
      "persist_messages": true
    }
  }'

# 2. Activar canal
curl -X PUT http://localhost:8080/api/v1/channels/{channel_id}/start

# 3. Pantallas se conectan automáticamente
# 4. Recepcionista llama turnos
curl -X POST http://localhost:8080/api/v1/channels/{channel_id}/broadcast \
  -d '{"content": "A004", "message_type": "Broadcast"}'
```

### **🏢 Alertas Corporativas**

**Escenario**: Empresa con alertas críticas por departamento.

```bash
# Canal por departamento
curl -X POST http://localhost:8080/api/v1/channels \
  -d '{
    "name": "Alertas IT",
    "settings": {
      "max_connections": 200,
      "allow_client_messages": true,
      "rate_limit_per_minute": 10
    }
  }'

# Sistema de monitoreo envía alertas
curl -X POST http://localhost:8080/api/v1/channels/{channel_id}/broadcast \
  -d '{"content": "🚨 Servidor principal caído", "message_type": "Broadcast"}'
```

### **📊 Dashboard en Tiempo Real**

**Escenario**: Métricas de ventas actualizándose en vivo.

```javascript
// Cliente Dashboard
const ws = new WebSocket(`ws://localhost:8080/api/v1/channels/${metricsChannelId}/ws`);

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    
    if (data.status === 'broadcast') {
        const metrics = JSON.parse(data.message);
        updateDashboard(metrics);
    }
};

// Sistema de ventas envía updates cada minuto
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

### **🎮 Notificaciones de Juego**

**Escenario**: Notificar eventos en tiempo real a jugadores.

```bash
# Canal para eventos del juego
curl -X POST http://localhost:8080/api/v1/channels \
  -d '{
    "name": "Game Events",
    "settings": {
      "max_connections": 10000,
      "allow_client_messages": false,
      "persist_messages": false
    }
  }'

# Evento: nuevo jugador se une
curl -X POST http://localhost:8080/api/v1/channels/{channel_id}/broadcast \
  -d '{"content": "🎉 Player MasterGamer joined the server!", "message_type": "Broadcast"}'
```

---

## 🧪 Ejemplos Prácticos

### **Cliente Dart (Flutter)**

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
        print('🎯 Turno llamado: ${data['message']}');
        showTurnNotification(data['message']);
        break;
      case 'connected':
        print('✅ Conectado al canal');
        break;
      default:
        print('📨 Mensaje: ${data['message']}');
    }
  }
  
  void showTurnNotification(String turn) {
    // Mostrar notificación en Flutter
    // Reproducir sonido
    // Actualizar UI
  }
  
  void disconnect() {
    _channel?.sink.close();
  }
}
```

### **Cliente React (JavaScript)**

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
      console.log('🟢 Conectado a EmitHub');
    };

    websocket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      
      setMessages(prev => [...prev, data]);
      
      if (data.status === 'broadcast') {
        // Mostrar notificación toast
        showNotification(data.message);
        
        // Reproducir sonido
        playNotificationSound();
      }
    };

    websocket.onclose = () => {
      setConnectionStatus('disconnected');
      console.log('🔴 Desconectado de EmitHub');
      
      // Reconectar automáticamente
      setTimeout(connectWebSocket, 5000);
    };

    setWs(websocket);
  };

  const showNotification = (message) => {
    // Usar biblioteca de notificaciones como react-toast
    toast.success(`🎯 Turno: ${message}`, {
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
        Estado: {connectionStatus === 'connected' ? '🟢 Conectado' : '🔴 Desconectado'}
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

### **Cliente Python**

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
            logging.info(f"🟢 Conectado a EmitHub: {uri}")
            
            await self.listen_messages()
            
        except Exception as e:
            logging.error(f"❌ Error conectando: {e}")
            
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
            print(f"🎯 Turno llamado: {message}")
            # Aquí puedes agregar lógica específica
            # como enviar email, actualizar base de datos, etc.
            
        elif status == 'connected':
            print(f"✅ Conectado al canal: {data.get('channel_id')}")
            
        else:
            print(f"📨 Mensaje: {message}")
            
    async def disconnect(self):
        if self.websocket:
            await self.websocket.close()

# Uso
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

### **Admin Panel (Gestión de Canales)**

```javascript
// Funciones para gestionar canales desde panel de administración

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

// Ejemplo de uso en panel de admin
const admin = new EmitHubAdmin('http://localhost:8080');

// Crear canal para turnos médicos
const channel = await admin.createChannel({
  name: 'Consultorio Cardiólogo',
  description: 'Canal para turnos de cardiología',
  settings: {
    max_connections: 100,
    allow_client_messages: false,
    persist_messages: true,
    rate_limit_per_minute: 30
  }
});

// Activar el canal
await admin.startChannel(channel.id);

// Enviar turno
await admin.broadcastMessage(channel.id, 'A015');
```

---

## 🐳 Deploy

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