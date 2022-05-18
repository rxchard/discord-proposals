use serenity::prelude::TypeMapKey;
use std::{collections::HashMap, io::Result, sync::Arc};
use tokio::{fs, io::AsyncWriteExt, sync::RwLock};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Store {
    #[serde(default)]
    pub channels: HashMap<u64, u64>,
}

#[derive(Debug)]
pub struct DataStore {
    path: String,
    pub store: Store,
}

impl DataStore {
    /// Tries to read the given path as JSON and serializes it.
    ///
    pub async fn new(path: &str) -> Result<Self> {
        // file or default, if the file does not exist then it will be created during the next save
        let file = match fs::read_to_string(path).await {
            Ok(file) => file,
            Err(_) => String::from("{}"),
        };

        Ok(Self {
            path: String::from(path),
            store: serde_json::from_str(&file)?,
        })
    }

    /// Deserializes `store` and writes it to a file at `self.path`
    ///
    pub async fn save(&self) -> Result<()> {
        let mut file = fs::File::create(&self.path).await?;

        file.write_all(serde_json::to_string(&self.store)?.as_bytes())
            .await
    }
}

impl TypeMapKey for DataStore {
    type Value = Arc<RwLock<DataStore>>;
}
