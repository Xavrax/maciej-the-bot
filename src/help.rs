pub fn help_message(prefix: &str) -> String {
    include_str!("../help.txt").replace("{}", prefix).into()
}
