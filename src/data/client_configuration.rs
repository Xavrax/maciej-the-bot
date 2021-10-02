use serenity::prelude::TypeMapKey;
use std::sync::Arc;

use tokio::sync::RwLock;

pub struct ClientConfiguration {
    pub prefix: String,
}

impl ClientConfiguration {
    pub fn new(prefix: String) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            prefix
        }))
    }
}

impl TypeMapKey for ClientConfiguration {
    type Value = Arc<RwLock<ClientConfiguration>>;
}