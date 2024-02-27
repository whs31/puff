#[derive(clap::Parser, Debug)]
pub struct Args
{
  /// Print poppy version
  #[arg(short, long)] pub version: bool,

  /// Install dependencies from manifest in current working folder
  #[arg(short, long)] pub install: bool
}