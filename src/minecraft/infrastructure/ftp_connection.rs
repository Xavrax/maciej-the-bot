use crate::minecraft::infrastructure::whitelist::Whitelist;
use crate::minecraft::MinecraftError;
use ftp::FtpStream;

pub struct FtpConnection {
    stream: FtpStream,
}

impl FtpConnection {
    pub fn new(ip: String, login: String, passwd: String) -> Result<Self, MinecraftError> {
        let mut stream =
            FtpStream::connect(ip).map_err(|e| return MinecraftError::FtpError(e.to_string()))?;

        stream.login(&login, &passwd);

        Ok(Self { stream })
    }
}

impl Drop for FtpConnection {
    fn drop(&mut self) {
        self.stream
            .quit()
            .expect("Ftp connection cannot be closed!");
    }
}

impl Whitelist for FtpConnection {
    fn add_to_whitelist(&self, nick: String) -> Result<(), MinecraftError> {
        Ok(())
    }
}
