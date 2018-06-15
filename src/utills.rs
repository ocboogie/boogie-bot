use colors::{ERROR_COLOR, SUCCESS_COLOR, WARN_COLOR};
use serenity::model::id::ChannelId;
use std::fmt::Display;

#[macro_export]
macro_rules! chain(
    ($y:expr; $( $x:expr ),*) => (
        {
          $(
            $y = $x;
          )*
          $y
        }
    );
);

#[allow(dead_code)]
pub fn report_error<D: Display>(channel: ChannelId, message: D) {
  let _ = channel.send_message(|m| {
    m.embed(|e| {
      e.color(ERROR_COLOR)
        .title("Error! 😞 ")
        .description(message)
    })
  });
}

pub fn warn<D: Display>(channel: ChannelId, message: D, title: Option<&str>) {
  let _ = channel.send_message(|m| {
    m.embed(|e| {
      e.color(WARN_COLOR)
        .title(title.unwrap_or("⚠"))
        .description(message)
    })
  });
}
pub fn success<D: Display>(channel: ChannelId, message: D, title: Option<&str>) {
  let _ = channel.send_message(|m| {
    m.embed(|e| {
      e.color(SUCCESS_COLOR)
        .title(title.unwrap_or("😀"))
        .description(message)
    })
  });
}
