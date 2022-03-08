use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Eq, PartialEq, Hash)]
pub enum ConfigRepositoryKey {
    FtpLogin,
    FtpPasswd,
    FtpIp,
}

pub struct ConfigRepository;

impl TypeMapKey for ConfigRepository {
    type Value = Arc<RwLock<HashMap<ConfigRepositoryKey, String>>>;
}

impl ConfigRepository {
    pub fn create(
        ftp_ip: String,
        ftp_login: String,
        ftp_passwd: String,
    ) -> Arc<RwLock<HashMap<ConfigRepositoryKey, String>>> {
        let mut map = HashMap::new();

        if !ftp_login.is_empty() && !ftp_passwd.is_empty() {
            map.insert(ConfigRepositoryKey::FtpLogin, ftp_login);
            map.insert(ConfigRepositoryKey::FtpPasswd, ftp_passwd);
            map.insert(ConfigRepositoryKey::FtpIp, ftp_ip);
        }

        Arc::new(RwLock::new(map))
    }
}
