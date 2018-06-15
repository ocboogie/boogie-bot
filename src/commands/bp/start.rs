use super::utills::get_points;
use prelude::*;
use serenity::framework::standard::CreateGroup;
use store::UserInfo;
use store::UsersInfo;
use utills::success;
use PREFIX;

struct StartCommand;

impl Command for StartCommand {
  fn execute(&self, ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let found_points = match get_points(ctx, msg.author.id) {
      Some(points) => Some(points),
      None => {
        let mut data = ctx.data.lock();
        let users = data.get_mut::<UsersInfo>().unwrap();
        users.insert(msg.author.id.0, UserInfo::default());
        None
      }
    };

    match found_points {
      Some(points) => {
        let _ = success(
          msg.channel_id,
          &format!("You've already started. You have {} points", points),
          None,
        );
      }
      None => {
        let _ = success(
          msg.channel_id,
          &format!(
            "You are now collecting Boogie Points! You can type `{}bp info` to see how many Boogie Points you have",
            PREFIX
          ),
          None,
        );
      }
    }

    Ok(())
  }
}

pub fn register_command(sf: CreateGroup) -> CreateGroup {
  sf.cmd("start", StartCommand)
}
