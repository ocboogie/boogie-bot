#[macro_use]
extern crate log;
// #[macro_use]
// extern crate emoji_converter;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate serde_json;
extern crate serenity;
extern crate timeago;
extern crate typemap;

use dotenv::dotenv;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::HashMap;
use std::env;
use store::UsersInfo;

#[macro_use]
mod utills;
mod colors;
mod commands;
mod prelude;
mod store;

pub static PREFIX: &'static str = ">";

struct Handler;

impl EventHandler for Handler {
  fn ready(&self, ctx: Context, ready: Ready) {
    info!("{} is connected!", ready.user.name);
    ctx.set_game_name(&format!("Type `{}help` for help.", PREFIX));
  }
}

fn main() {
  dotenv().ok();
  env_logger::init();
  let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
    .expect("Error creating client");

  {
    let mut data = client.data.lock();
    data.insert::<UsersInfo>(HashMap::default());
    match store::UsersInfo::load() {
      Ok(map) => {
        data.insert::<UsersInfo>(map);
      }
      Err(why) => {
        error!("Err loading user points: {:?}", why);
      }
    }
  }

  let mut sf = StandardFramework::new().configure(|c| c.prefix(PREFIX));

  sf = commands::register_commands(sf);

  client.with_framework(sf);

  if let Err(why) = client.start() {
    println!("An error occurred while running the client: {:?}", why);
  }
}
