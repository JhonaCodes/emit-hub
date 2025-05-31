use redb::{Database, ReadableTable};
use crate::models::channel::Channel;
use crate::state::CHANNELS_TABLE;

pub async fn inspect_database(db_path: &str) -> anyhow::Result<()> {
    let db = Database::open(db_path)?;
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(CHANNELS_TABLE)?;

    println!("Database contents:");
    for result in table.iter()? {
        let (key, value) = result?;
        let channel: Channel = serde_json::from_slice(value.value().as_ref())?;
        println!("Channel {}: {:?}", key.value(), channel);
    }

    Ok(())
}