use colored::Color::{Cyan, Red, Yellow};
use colored::Colorize;
use flexi_logger::{AdaptiveFormat, DeferredNow};
use log::{Level, Record};
use crate::consts::POPPY_NAME;

fn log_format(
  w: &mut dyn std::io::Write,
  now: &mut DeferredNow,
  record: &Record
) -> Result<(), std::io::Error>
{
  let level = record.level();
  write!(
    w,
    "{}: {}",
    match level {
      Level::Error => POPPY_NAME.color(Red).bold(),
      Level::Warn => POPPY_NAME.color(Yellow).bold(),
      _ => POPPY_NAME.color(Cyan).bold(),
    },
    match level {
      Level::Error => record.args().to_string().color(Red).bold(),
      Level::Warn => record.args().to_string().color(Yellow).bold(),
      Level::Info => record.args().to_string().color(Cyan).bold(),
      Level::Debug => record.args().to_string().bold(),
      Level::Trace => record.args().to_string().into(),
    }
  )
}

pub fn init_cli_logger(level: &str) -> anyhow::Result<()>
{
  flexi_logger::Logger::try_with_env_or_str(level)?
    .format_for_stdout(log_format)
    .format_for_stderr(log_format)
    .start()?;
  Ok(())
}