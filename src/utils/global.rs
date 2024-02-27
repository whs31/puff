use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::consts::POPPY_NAME;

lazy_static!
{
  pub static ref PROJECT_DIRS: Mutex<directories::ProjectDirs> = Mutex::new(
    directories::ProjectDirs::from("org", "poppy", POPPY_NAME).unwrap()
  );
}