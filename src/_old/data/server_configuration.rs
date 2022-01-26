use serenity::prelude::TypeMapKey;
use std::sync::Arc;

use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Copy, Clone)]
pub enum Feature {
    YouTube,
    BusyToday,
}

#[derive(Default)]
pub struct ServerConfiguration {
    pub features: Vec<Feature>,
}

pub struct ServersConfiguration(pub HashMap<u32, ServerConfiguration>);

impl ServersConfiguration {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self(HashMap::new())))
    }
}

impl TypeMapKey for ServersConfiguration {
    type Value = Arc<RwLock<ServersConfiguration>>;
}
