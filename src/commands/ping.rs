use prelude::*;

struct PingCommand;

impl Command for PingCommand {
  fn execute(&self, _ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let _ = msg.reply("Pong!");
    Ok(())
  }
}

pub fn register_command(sf: StandardFramework) -> StandardFramework {
  sf.cmd("ping", PingCommand)
}
