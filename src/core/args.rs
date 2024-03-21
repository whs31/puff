#[derive(clap::Parser, Debug, Clone)]
#[command(name = "parcel")]
#[command(about = "parcel - a tool for managing c/c++ packages", long_about = None)]
#[command(color = clap::ColorChoice::Auto)]
pub struct Args
{
  #[command(subcommand)]
  pub command: Option<Command>
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command
{
  #[clap(subcommand)]
  Registry(RegistryCommand),
}

// parcel registry add --name [name] --url [url] --pattern [pattern] --username [username] --token [token]

#[derive(clap::Subcommand, Debug, Clone)]
pub enum RegistryCommand
{
  Add(RegistryAddArgs),
  Remove(RegistryRemoveArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub struct RegistryAddArgs
{
  #[arg(short, long)] pub name: String,
  #[arg(long)] pub url: String,
  #[arg(short, long)] pub pattern: String,
  #[arg(short, long)] pub username: Option<String>,
  #[arg(short, long)] pub token: Option<String>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct RegistryRemoveArgs
{
  #[arg(short, long)] pub name: String,
}