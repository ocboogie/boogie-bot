use super::utills::{get_points, started_check};
use colors::SUCCESS_COLOR;
use prelude::*;
use serenity::framework::standard::CreateGroup;
use store::UsersInfo;
use timeago::Formatter;

struct InfoCommand;

impl Command for InfoCommand {
  fn execute(&self, ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let collecting_sense = {
      let mut data = ctx.data.lock();
      let users = data.get_mut::<UsersInfo>().unwrap();
      let user = users.get(&msg.author.id.0).unwrap();
      let formatter = Formatter::new();
      formatter.convert(user.collecting_sinse.elapsed().unwrap())
    };
    let _ = msg.channel_id.send_message(|m| {
      m.embed(|e| {
        e.color(SUCCESS_COLOR)
          .field(
            "Boogie Points",
            get_points(ctx, msg.author.id).unwrap().to_string(),
            true,
          )
          .field("Collecting sense", collecting_sense, true)
      })
    });
    // let _ = success(
    //   msg.channel_id,
    //   format!("You have `{}` Boogie Points", get_points(ctx, msg.author.id).unwrap().to_string()),
    //   Some("😏"),
    // );
    // let _ = msg.reply(&match get_points(ctx, msg.author.id) {
    //   Some(points) => points.to_string(),
    //   None => format!(
    //     "You've not started collecting Boogie Points. You can start by typing: `{}bp start`",
    //     PREFIX
    //   ),
    // });

    Ok(())
  }
}

pub fn register_command(sf: CreateGroup) -> CreateGroup {
  sf.command("info", |c| c.check(started_check).cmd(InfoCommand))
}
