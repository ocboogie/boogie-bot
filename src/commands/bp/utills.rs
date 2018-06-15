use prelude::*;
use serenity::framework::standard::CommandOptions;
use serenity::model::id::UserId;
use std::time::SystemTime;
use store::UsersInfo;
use PREFIX;

pub fn get_points(context: &mut Context, user_id: UserId) -> Option<usize> {
  let mut data = context.data.lock();
  let users = data.get_mut::<UsersInfo>().unwrap();
  let points = {
    let user = users.get_mut(&user_id.0)?;

    user.points += user
      .last_check
      .elapsed()
      .map_err(|why| error!("error getting points: {:?}", why))
      .ok()?
      .as_secs() as usize;

    user.last_check = SystemTime::now();

    user.points
  };

  if let Err(why) = UsersInfo::save(users) {
    warn!("Error saving points: {:?}", why);
  }

  Some(points)
}

pub fn started_check(
  context: &mut Context,
  message: &Message,
  _: &mut Args,
  _: &CommandOptions,
) -> bool {
  let has_user = {
    let mut data = context.data.lock();
    let users = data.get_mut::<UsersInfo>().unwrap();
    users.contains_key(&message.author.id.0)
  };
  if has_user {
    true
  } else {
    let _ = message.reply(&format!(
      "You haven't started collecting Boogie Points. You can start by typing: `{}bp start`",
      PREFIX
    ));
    false
  }
}
