#[derive(clap::Parser, Debug, Clone)]
#[command(name = "parcel")]
#[command(about = "parcel - a tool for managing c/c++ packages", long_about = None)]
#[command(color = clap::ColorChoice::Auto)]
pub struct Args
{
  /// Execute one of major subcommands
  #[command(subcommand)] pub command: Option<Command>
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command
{
  /// Build package
  Build(BuildArgs),

  /// Add or remove a registry from puff
  #[clap(subcommand)] Registry(RegistryCommand),

  /// Specify options for toolchains
  #[clap(subcommand)] Toolchain(ToolchainCommand),

  /// Pack package into a tarball
  Pack(PackArgs),
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

  /// Package layout pattern in selected repository in Artifactory
  #[arg(short, long)] pub pattern: String,

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
}

#[derive(clap::Args, Debug, Clone)]
pub struct PackArgs
{
  /// Folder where manifest is located
  pub folder: Option<String>,

  /// Output path
  pub output: Option<String>
}