use super::utills::{get_points, started_check};
use prelude::*;
use serenity::framework::standard::CreateGroup;
use store::UsersInfo;
use utills::{success, warn};

struct GiveCommand;

impl Command for GiveCommand {
  fn execute(&self, ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let target = match msg.mentions.first() {
      Some(user) => user,
      None => {
        warn(msg.channel_id, "You must mention a user", Some("❓👤❓"));
        return Ok(());
      }
    };

    let give_points = match args.find::<usize>() {
      Ok(points) => points,
      Err(_) => {
        warn(msg.channel_id, "You must supply an amount of points", Some("❓💵❓"));
        return Ok(());
      }
    };

    // if target.id == msg.author.id {
    //   let _ = warn(
    //     msg.channel_id,
    //     "You can't give yourself points.",
    //     Some("🤦"),
    //   );
    // }

    let user_points = get_points(ctx, msg.author.id).unwrap();
    if user_points < give_points {
      let _ = warn(msg.channel_id, "You don't have enough points", None);
      return Ok(());
    };

    {
      let mut data = ctx.data.lock();
      let users = data.get_mut::<UsersInfo>().unwrap();
      {
        let target_user_points = match users.get_mut(&target.id.0) {
          Some(target_user_points) => target_user_points,
          None => {
            let _ = warn(
              msg.channel_id,
              "They haven't started collecting Boogie Points yet",
              None,
            );
            return Ok(());
          }
        };
        target_user_points.points += give_points;
      }
      let giver_user_points = users.get_mut(&msg.author.id.0).unwrap();
      giver_user_points.points -= give_points;
    };

    success(
      msg.channel_id,
      format!(
        "You've successfully given `{}` Boogie Points to <@{}>",
        give_points, target.id
      ),
      Some("🤑 ➡️ 😌"),
    );

    // let _ = msg.channel_id.send_message(|m| {
    //   m.embed(|e| {
    //     e.color(WARN_COLOR).title("😟️").description(format!(
    //       "Are you sure you want to give {} to <@{}>",
    //       give_points, target.id
    //     ))
    //   }).reactions(vec!['✅', '❌'])
    // });

    Ok(())
  }
}

pub fn register_command(sf: CreateGroup) -> CreateGroup {
  sf.command("give", |c| c.check(started_check).cmd(GiveCommand).desc("Gives a set amount of points to someone.").example("@someone 100").usage("<someone> <amount of points>"))
}
