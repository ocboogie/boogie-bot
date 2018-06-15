use prelude::*;

mod utills;

mod give;
mod info;
mod start;

pub fn register_command(sf: StandardFramework) -> StandardFramework {
  sf.group("bp", |mut g| {
    chain!(g;
      give::register_command(g),
      info::register_command(g),
      start::register_command(g)
    ).prefix("bp")
  })
}
