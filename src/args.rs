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

  /// Set username for artifactory OAuth. Use --token to set token.
  #[arg(short, long)] pub username: Option<String>,

  /// Set token for artifactory OAuth. Use --username to set username.
  #[arg(short, long)] pub token: Option<String>,

  /// Verbose output
  #[arg(short, long)] pub verbose: bool,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Commands
{
  /// Install dependencies from manifest in current working folder
  Install(InstallArgs),

  /// Push file to artifactory
  Push(PushArgs),

  /// Clean dependencies folder
  Clean,

  /// Sync with remote registry
  Sync(SyncArgs),

  /// Purge cache and config folders. Specify --cache to clear only cache or --all to clear all entries
  Purge(PurgeArgs),

  /// Get specified field from manifest in current working folder
  Parse { what: Option<String> },
}

#[derive(clap::Args, Debug, Clone)]
pub struct PurgeArgs
{
  /// Specify for purge to clear only cache
  #[arg(long)] pub cache: bool,

  /// Specify for purge to clear only config
  #[arg(long)] pub config: bool,

  /// Specify for purge to clear all (config + cache)
  #[arg(long)] pub all: bool
}

#[derive(clap::Args, Debug, Clone)]
pub struct InstallArgs
{
  /// Perform installation in lazy mode (no implicit git clone/downloading)
  #[arg(short, long)] pub lazy: bool,

  /// Sync remote registry
  #[arg(short, long)] pub sync: bool,

  /// List remote registry
  #[arg(long)] pub list_registry: bool,

  /// Clean dependencies folder and continue fresh installation
  #[arg(short, long)] pub fresh: bool,

  /// Override platform arch (default: native)
  #[arg(short, long)] pub arch: Option<String>,

  /// If this option is specified, poppy will use the exact version specified in the manifest. No dependency round-up will be used.
  #[arg(short, long)] pub exact_version: bool
}

#[derive(clap::Args, Debug, Clone)]
pub struct PushArgs
{
  /// REQUIRED: File to push to artifactory
  pub name: Option<String>,

  /// Force push (override existing package on artifactory)
  #[arg(short, long)] pub force: bool,

  /// REQUIRED: Specify distribution to push to artifactory
  #[arg(short, long)] pub distribution: Option<String>,

  /// REQUIRED: Specify platform arch to push to artifactory
  #[arg(short, long)] pub arch: Option<String>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct SyncArgs
{
  /// Sync and list remote registry
  #[arg(short, long)] pub registry: bool,

  /// Download all blobs from remote registry into cache
  #[arg(short, long)] pub cache_all: bool,

  /// Include poppy packages in --cache-all routine
  #[arg(long)] pub include_self: bool
}