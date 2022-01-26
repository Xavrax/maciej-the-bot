use crate::presentation::commands::help::{HELP_COMMAND, H_COMMAND};
use serenity::framework::standard::macros::group;

#[group]
#[only_in(guilds)]
#[commands(h, help)]
struct General;

// #[group]
// #[only_in(guilds)]
// #[prefixes("op")]
// #[commands(ophelp)]
// struct Operator;
