use serenity::prelude::TypeMapKey;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}