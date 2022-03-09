use crate::minecraft::MinecraftError;

pub trait Whitelist {
    fn add_to_whitelist(&self, nick: String) -> Result<(), MinecraftError>;
}
