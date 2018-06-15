use prelude::*;

struct ReactCommand;

impl Command for ReactCommand {
  fn execute(&self, _ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let _ = msg.react("1⃣");
    let _ = msg.react("2⃣");
    let _ = msg.react("3⃣");
    Ok(())
  }
}

pub fn register_command(sf: StandardFramework) -> StandardFramework {
  sf.cmd("react", ReactCommand)
}
