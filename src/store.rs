use failure::Error;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use typemap::Key;

static CONFIG_NAME: &'static str = "boogie_points.json";

pub struct UsersInfo;

impl UsersInfo {
  pub fn load() -> Result<HashMap<u64, UserInfo>, Error> {
    let mut s = String::new();

    let path = Path::new(CONFIG_NAME);

    let mut f = if path.exists() {
      File::open(path)?
    } else {
      let mut f = File::create(path)?;

      if let Err(why) = f.write(b"{}") {
        error!("Err writing to new file: {}", why);
      }

      File::open(path)?
    };

    f.read_to_string(&mut s)?;

    Ok(serde_json::from_str::<HashMap<u64, UserInfo>>(&s)?)
  }

  pub fn save(map: &HashMap<u64, UserInfo>) -> Result<(), Error> {
    let body = serde_json::to_string(map)?;

    let mut f = File::create(CONFIG_NAME)?;
    let _ = f.write(body.as_bytes())?;

    Ok(())
  }
}

impl Key for UsersInfo {
  type Value = HashMap<u64, UserInfo>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
  pub points: usize,
  pub collecting_sinse: SystemTime,
  // #[serde(skip, default = "SystemTime::now")]
  pub last_check: SystemTime,
}

impl Default for UserInfo {
  fn default() -> Self {
    UserInfo {
      points: 100,
      collecting_sinse: SystemTime::now(),
      last_check: SystemTime::now(),
    }
  }
}
