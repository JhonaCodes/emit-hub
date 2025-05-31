use serde::{Deserialize, Serialize};
use std::env;

/// Configuración principal de EmitHub
/// Puede ser cargada desde variables de entorno o archivo de configuración
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Host donde el servidor escuchará conexiones
    pub host: String,

    /// Puerto donde el servidor escuchará conexiones
    pub port: u16,

    /// Ruta al archivo de base de datos ReDB
    pub db_path: String,

    /// Número máximo de conexiones WebSocket por canal
    pub max_connections_per_channel: usize,

    /// Tamaño máximo de mensaje en bytes (1MB por defecto)
    pub message_size_limit: usize,

    /// Configuración de logging
    pub log_level: String,

    /// Configuración de CORS
    pub cors: CorsConfig,

    /// Configuración de WebSocket
    pub websocket: WebSocketConfig,

    /// Configuración de persistencia
    pub persistence: PersistenceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Orígenes permitidos para CORS (usar "*" para todos)
    pub allowed_origins: Vec<String>,

    /// Métodos HTTP permitidos
    pub allowed_methods: Vec<String>,

    /// Headers permitidos
    pub allowed_headers: Vec<String>,

    /// Tiempo de caché para preflight requests en segundos
    pub max_age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// Timeout para conexiones WebSocket en segundos
    pub connection_timeout: u64,

    /// Tamaño máximo del buffer de continuación
    pub max_continuation_size: usize,

    /// Intervalo de ping en segundos (0 = deshabilitado)
    pub ping_interval: u64,

    /// Timeout de pong en segundos
    pub pong_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Si persistir mensajes por defecto
    pub persist_messages_default: bool,

    /// Número máximo de mensajes a guardar por canal
    pub max_messages_per_channel: usize,

    /// Días a mantener mensajes antes de limpiarlos
    pub message_retention_days: u32,

    /// Si hacer backup automático de la base de datos
    pub auto_backup: bool,

    /// Intervalo de backup en horas
    pub backup_interval_hours: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            db_path: "emit_hub.redb".to_string(),
            max_connections_per_channel: 1000,
            message_size_limit: 1_048_576, // 1MB
            log_level: "info".to_string(),
            cors: CorsConfig::default(),
            websocket: WebSocketConfig::default(),
            persistence: PersistenceConfig::default(),
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "http://localhost:8080".to_string(),
                "http://127.0.0.1:3000".to_string(),
                "http://127.0.0.1:8080".to_string(),
            ],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
            ],
            allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "Accept".to_string(),
                "Origin".to_string(),
                "X-Requested-With".to_string(),
            ],
            max_age: 3600, // 1 hora
        }
    }
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            connection_timeout: 30,
            max_continuation_size: 2_usize.pow(20), // 1MB
            ping_interval: 30,
            pong_timeout: 10,
        }
    }
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            persist_messages_default: false,
            max_messages_per_channel: 10_000,
            message_retention_days: 30,
            auto_backup: false,
            backup_interval_hours: 24,
        }
    }
}

impl Config {
    /// Cargar configuración desde variables de entorno
    pub fn from_env() -> anyhow::Result<Self> {
        let mut config = Config::default();

        // Configuración básica del servidor
        if let Ok(host) = env::var("EMIT_HUB_HOST") {
            config.host = host;
        }

        if let Ok(port) = env::var("EMIT_HUB_PORT") {
            config.port = port.parse().map_err(|e| {
                anyhow::anyhow!("Invalid port number '{}': {}", port, e)
            })?;
        }

        if let Ok(db_path) = env::var("EMIT_HUB_DB_PATH") {
            config.db_path = db_path;
        }

        if let Ok(max_conn) = env::var("EMIT_HUB_MAX_CONNECTIONS") {
            config.max_connections_per_channel = max_conn.parse().map_err(|e| {
                anyhow::anyhow!("Invalid max connections '{}': {}", max_conn, e)
            })?;
        }

        if let Ok(msg_size) = env::var("EMIT_HUB_MESSAGE_SIZE_LIMIT") {
            config.message_size_limit = msg_size.parse().map_err(|e| {
                anyhow::anyhow!("Invalid message size limit '{}': {}", msg_size, e)
            })?;
        }

        if let Ok(log_level) = env::var("EMIT_HUB_LOG_LEVEL") {
            config.log_level = log_level;
        }

        // Configuración CORS
        if let Ok(origins) = env::var("EMIT_HUB_CORS_ORIGINS") {
            config.cors.allowed_origins = origins
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        // Configuración WebSocket
        if let Ok(timeout) = env::var("EMIT_HUB_WS_TIMEOUT") {
            config.websocket.connection_timeout = timeout.parse().map_err(|e| {
                anyhow::anyhow!("Invalid WebSocket timeout '{}': {}", timeout, e)
            })?;
        }

        if let Ok(ping_interval) = env::var("EMIT_HUB_WS_PING_INTERVAL") {
            config.websocket.ping_interval = ping_interval.parse().map_err(|e| {
                anyhow::anyhow!("Invalid ping interval '{}': {}", ping_interval, e)
            })?;
        }

        // Configuración de persistencia
        if let Ok(persist) = env::var("EMIT_HUB_PERSIST_MESSAGES") {
            config.persistence.persist_messages_default = persist.parse().map_err(|e| {
                anyhow::anyhow!("Invalid persist messages value '{}': {}", persist, e)
            })?;
        }

        if let Ok(retention) = env::var("EMIT_HUB_MESSAGE_RETENTION_DAYS") {
            config.persistence.message_retention_days = retention.parse().map_err(|e| {
                anyhow::anyhow!("Invalid retention days '{}': {}", retention, e)
            })?;
        }

        if let Ok(auto_backup) = env::var("EMIT_HUB_AUTO_BACKUP") {
            config.persistence.auto_backup = auto_backup.parse().map_err(|e| {
                anyhow::anyhow!("Invalid auto backup value '{}': {}", auto_backup, e)
            })?;
        }

        // Validar configuración
        config.validate()?;

        Ok(config)
    }

    /// Validar que la configuración sea correcta
    fn validate(&self) -> anyhow::Result<()> {
        if self.port == 0 {
            return Err(anyhow::anyhow!("Port cannot be 0"));
        }

        if self.max_connections_per_channel == 0 {
            return Err(anyhow::anyhow!("Max connections per channel must be greater than 0"));
        }

        if self.message_size_limit == 0 {
            return Err(anyhow::anyhow!("Message size limit must be greater than 0"));
        }

        if self.websocket.connection_timeout == 0 {
            return Err(anyhow::anyhow!("WebSocket connection timeout must be greater than 0"));
        }

        if self.persistence.message_retention_days == 0 {
            return Err(anyhow::anyhow!("Message retention days must be greater than 0"));
        }

        // Validar que el directorio de la base de datos exista o se pueda crear
        if let Some(parent) = std::path::Path::new(&self.db_path).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    anyhow::anyhow!("Cannot create database directory {:?}: {}", parent, e)
                })?;
            }
        }

        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.max_connections_per_channel, 1000);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();

        // Configuración válida
        assert!(config.validate().is_ok());

        // Puerto inválido
        config.port = 0;
        assert!(config.validate().is_err());

        // Restaurar y probar max_connections
        config.port = 8080;
        config.max_connections_per_channel = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_env_config() {
        struct EnvGuard;
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                unsafe {
                    env::remove_var("EMIT_HUB_HOST");
                    env::remove_var("EMIT_HUB_PORT");
                    env::remove_var("EMIT_HUB_MAX_CONNECTIONS");
                }
            }
        }
        let _guard = EnvGuard;

        unsafe {
            env::set_var("EMIT_HUB_HOST", "0.0.0.0");
            env::set_var("EMIT_HUB_PORT", "9000");
            env::set_var("EMIT_HUB_MAX_CONNECTIONS", "2000");
        }

        let config = Config::from_env().unwrap();

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 9000);
        assert_eq!(config.max_connections_per_channel, 2000);
    }
}