use log::{debug, error, info, trace, warn};
use crate::utils::config::CONFIG;

pub fn run_poppy() -> Result<(), anyhow::Error>
{
  let _w = CONFIG.lock().unwrap();
  Ok(())
}