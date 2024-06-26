#[derive(clap::Parser, Debug, Clone)]
#[command(name = "puff", bin_name = "puff")]
#[command(about = "puff - a tool for managing c/c++ packages", long_about = None)]
#[command(color = clap::ColorChoice::Auto)]
pub struct Args
{
  /// Execute one of major subcommands
  #[command(subcommand)] pub command: Option<Command>,

  /// Print version and exit
  #[arg(short, long)] pub version: bool
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command
{
  // Build package (todo)
  // Build(BuildArgs),

  /// Install required dependencies
  Install(InstallArgs),

  /// Add or remove a registry from puff
  #[clap(subcommand)] Registry(RegistryCommand),

  /// Specify options for toolchains
  #[clap(subcommand)] Toolchain(ToolchainCommand),

  /// Pack package into a tarball
  Pack(PackArgs),

  /// Pack and push package to Artifactory
  Publish(PublishArgs),

  /// Purge selected local folders
  Purge(PurgeArgs),
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum RegistryCommand
{
  /// Add a new registry to parcel
  Add(RegistryAddArgs),

  /// Remove an existing registry from parcel
  Remove(RegistryRemoveArgs),

  /// List all registries in parcel
  List,
}

#[derive(clap::Args, Debug, Clone)]
pub struct RegistryAddArgs
{
  /// Name of the registry to be added. Must be same as the name of the repository in Artifactory
  #[arg(short, long)] pub name: String,

  /// URL of the Artifactory registry, stripped of any trailing slashes and without repository name
  #[arg(long)] pub url: String,

  /// Package layout pattern in selected repository in Artifactory. May lead to errors and bugs, use with caution
  #[arg(short, long)] pub pattern: Option<String>,

  /// Username for basic auth in Artifactory
  #[arg(short, long)] pub username: Option<String>,

  /// Token for basic auth in Artifactory
  #[arg(short, long)] pub token: Option<String>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct RegistryRemoveArgs
{
  /// Name of the registry to remove
  #[arg(short, long)] pub name: String,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum ToolchainCommand
{
  /// Specify options for CMake toolchain
  Cmake(ToolchainCmakeArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub struct ToolchainCmakeArgs
{
  /// Override CMake configure-step binary to use. Useful for cross-compiling (e.g., `x86_64-w64-mingw32-cmake` for Windows).
  #[arg(long)] pub configure_command: Option<String>,

  /// Additional CMake configure-step arguments. Useful for cross-compiling (e.g., `--target=x86_64-w64-mingw32` for Windows).
  #[arg(long)]
  #[clap(num_args = 0.., value_delimiter = ',')]
  pub configure_args: Option<Vec<String>>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct BuildArgs
{
  /// Folder where manifest is located
  pub folder: Option<String>,

  /// Type of distribution
  #[arg(short, long)] pub dist: crate::types::Distribution,
}

#[derive(clap::Args, Debug, Clone)]
pub struct InstallArgs
{
  /// Folder where manifest is located
  pub folder: Option<String>,

  /// Target OS
  #[arg(short, long)] pub os: Option<crate::types::OperatingSystem>,

  /// Target architecture
  #[arg(short, long)] pub arch: Option<crate::types::Arch>,

  /// Clean previously installed packages and perform fresh installation
  #[arg(short, long)] pub fresh: bool,

  /// Compile all packages from source
  #[arg(short, long)] pub source_only: bool
}

#[derive(clap::Args, Debug, Clone)]
pub struct PackArgs
{
  /// Folder where manifest is located
  pub folder: Option<String>,

  /// Output path
  #[arg(short, long)] pub output: Option<String>
}

#[derive(clap::Args, Debug, Clone)]
pub struct PublishArgs
{
  /// Folder where manifest is located
  pub folder: Option<String>,

  /// Type of distribution
  #[arg(short, long)] pub dist: Option<crate::types::Distribution>,

  /// Name of the registry to be added. Must be same as the name of the repository in Artifactory
  #[arg(short, long)] pub name: String,

  /// Package architecture
  #[arg(short, long)] pub arch: Option<crate::types::Arch>,

  /// Package operating system
  #[arg(short, long)] pub os: Option<crate::types::OperatingSystem>,

  /// Overwrite existing package
  #[arg(short, long)] pub force: bool
}

#[derive(clap::Args, Debug, Clone)]
pub struct PurgeArgs
{
  /// Purge all folders (implies --cache --config)
  #[arg(long)] pub all: bool,

  /// Purge cache folder (implies --all)
  #[arg(long)] pub cache: bool,

  /// Purge config folder (implies --all)
  #[arg(long)] pub config: bool
}