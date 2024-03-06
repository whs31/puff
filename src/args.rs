#[derive(clap::Parser, Debug, Clone)]
#[command(name = "poppy")]
#[command(about = "poppy - a tool for managing c/c++ packages", long_about = None)]
#[command(color = clap::ColorChoice::Auto)]
pub struct Args
{
  #[command(subcommand)]
  pub command: Option<Commands>,

  /// Print poppy version
  #[arg(long)] pub version: bool,

  /// Sync remote registry
  #[arg(short, long)] pub sync: bool,

  /// Override platform arch (default: native)
  #[arg(long)] pub arch: Option<String>,

  /// Perform operations in lazy mode (no implicit git clone/downloading)
  #[arg(short, long)] pub lazy: bool,

  /// Create and configure manifest in current working folder
  #[arg(long)] pub create: bool,

  /// Specify for purge to clear only cache
  #[arg(long)] pub cache_only: bool,

  /// Set username for artifactory OAuth. Use --token to set token.
  #[arg(long)] pub username: Option<String>,

  /// Set token for artifactory OAuth. Use --username to set username.
  #[arg(long)] pub token: Option<String>,

  /// Specify distribution to push to artifactory
  #[arg(long)] pub distribution: Option<String>,

  /// Force!
  #[arg(long)] pub force: bool,

  /// Clean dependencies folder and continue fresh installation
  #[arg(short, long)] pub fresh: bool,

  /// Verbose output
  #[arg(short, long)] pub verbose: bool,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Commands
{
  /// Install dependencies from manifest in current working folder
  Install,

  /// Push file to artifactory
  Push { name: Option<String> },

  /// Clean dependencies folder
  Clean,

  /// Purge cache and config folders
  Purge,

  /// Get specified field from manifest in current working folder
  Parse { what: Option<String> },
}