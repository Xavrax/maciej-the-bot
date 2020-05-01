pub fn say(msg : impl std::fmt::Display) -> String {
    format!("> {}", msg)
}