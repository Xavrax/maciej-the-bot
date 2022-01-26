pub mod prefix_configurable_help_service;

pub enum HelpLevel {
    User,
    Operator,
}

pub trait HelpService {
    fn help(&self, help_level: HelpLevel) -> String;
}
