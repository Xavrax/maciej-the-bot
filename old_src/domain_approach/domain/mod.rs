pub mod help;
pub mod music;

use songbird::input as songbird;

pub enum MusicInput {
    Dummy,
    Youtube(songbird::Input),
}
