use serenity::{
    prelude::{EventHandler, Context},
    model::prelude::Ready
};

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}