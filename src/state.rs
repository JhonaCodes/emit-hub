use crate::models::{channel::Channel, message::BroadcastMessage};
use actix_ws::Session;
use anyhow::Result;
use redb::{Database, ReadableTable, TableDefinition};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

const CHANNELS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("channels");
const MESSAGES_TABLE: TableDefinition<&str, &str> = TableDefinition::new("messages");

pub struct AppState {
    pub db: Arc<Database>,
    pub active_channels: Arc<RwLock<HashMap<Uuid, Channel>>>,
    pub connections: Arc<Mutex<HashMap<Uuid, Vec<Session>>>>,
}

impl AppState {
    pub async fn new(db_path: &str) -> Result<Self> {
        // Inicializar base de datos ReDB
        let db = Database::create(db_path)?;

        // Crear tablas si no existen
        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(CHANNELS_TABLE)?;
            let _ = write_txn.open_table(MESSAGES_TABLE)?;
        }
        write_txn.commit()?;

        let state = Self {
            db: Arc::new(db),
            active_channels: Arc::new(RwLock::new(HashMap::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
        };

        // Cargar canales activos desde la base de datos
        state.load_active_channels().await?;

        Ok(state)
    }

    async fn load_active_channels(&self) -> Result<()> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(CHANNELS_TABLE)?;

        let mut active_channels = self.active_channels.write().await;

        for result in table.iter()? {
            let (key, value) = result?;
            let channel: Channel = serde_json::from_str(value.value())?;

            // Solo cargar canales que estaban activos o pausados
            if matches!(channel.status, crate::models::channel::ChannelStatus::Active | crate::models::channel::ChannelStatus::Paused) {
                active_channels.insert(channel.id, channel);
            }
        }

        tracing::info!("Loaded {} active channels from database", active_channels.len());
        Ok(())
    }

    pub async fn save_channel(&self, channel: &Channel) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CHANNELS_TABLE)?;
            let channel_json = serde_json::to_string(channel)?;
            table.insert(channel.id.to_string().as_str(), channel_json.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub async fn save_message(&self, message: &BroadcastMessage) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(MESSAGES_TABLE)?;
            let message_json = serde_json::to_string(message)?;
            table.insert(message.id.to_string().as_str(), message_json.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub async fn get_channel(&self, channel_id: &Uuid) -> Option<Channel> {
        self.active_channels.read().await.get(channel_id).cloned()
    }

    pub async fn update_channel_status(&self, channel_id: Uuid, status: crate::models::channel::ChannelStatus) -> Result<()> {
        let mut active_channels = self.active_channels.write().await;

        if let Some(channel) = active_channels.get_mut(&channel_id) {
            channel.status = status;
            channel.updated_at = chrono::Utc::now();
            self.save_channel(channel).await?;
        }

        Ok(())
    }

    pub async fn add_connection(&self, channel_id: Uuid, session: Session) {
        let mut connections = self.connections.lock().await;
        connections.entry(channel_id).or_default().push(session);
    }

    pub async fn remove_connection(&self, channel_id: &Uuid, session: &Session) {
        let mut connections = self.connections.lock().await;
        if let Some(sessions) = connections.get_mut(channel_id) {
            sessions.retain(|s| !std::ptr::eq(s, session));
        }
    }

    pub async fn broadcast_to_channel(&self, channel_id: &Uuid, message: &str) -> Result<usize> {
        let mut connections = self.connections.lock().await;
        let mut sent_count = 0;

        if let Some(sessions) = connections.get_mut(channel_id) {
            let mut active_sessions = Vec::new();

            for session in sessions.iter_mut() {
                if session.text(message).await.is_ok() {
                    sent_count += 1;
                    active_sessions.push(session.clone());
                }
            }

            *sessions = active_sessions;
        }

        Ok(sent_count)
    }
    
}