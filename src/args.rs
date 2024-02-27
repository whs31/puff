#[derive(clap::Parser, Debug)]
pub struct Args
{
  /// Print poppy version
  #[arg(short, long)]pub version: bool
}