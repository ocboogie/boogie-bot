pub type CommandResult = Result<(), CommandError>;

pub use serenity::client::Context;
pub use serenity::framework::standard::{Args, Command, CommandError, StandardFramework};
pub use serenity::model::channel::Message;
