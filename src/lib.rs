pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub type Error = Box<dyn std::error::Error>;
