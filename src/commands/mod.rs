use serenity::framework::standard::StandardFramework;
use serenity::framework::standard::help_commands;

mod ping;
mod react;
// mod spell;
mod bp;

pub fn register_commands(mut sf: StandardFramework) -> StandardFramework {
  chain!(sf;
    ping::register_command(sf), 
    react::register_command(sf), 
    // spell::register_command(sf),
    bp::register_command(sf)
  ).help(help_commands::with_embeds)
}
