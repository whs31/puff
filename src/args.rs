#[derive(clap::Parser, Debug, Clone)]
pub struct Args
{
  /// Print poppy version
  #[arg(short, long)] pub version: bool,

  /// Install dependencies from manifest in current working folder
  #[arg(short, long)] pub install: bool,

  /// Sync remote registry
  #[arg(short, long)] pub sync: bool,

  /// Override platform arch (default: native)
  #[arg(short, long)] pub arch: Option<String>,

  /// Perform operations in lazy mode (no implicit git clone/downloading)
  #[arg(short, long)] pub lazy: bool,

  /// Create and configure manifest in current working folder
  #[arg(long)] pub create: bool,
}