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
  #[arg(long)] pub arch: Option<String>,

  /// Perform operations in lazy mode (no implicit git clone/downloading)
  #[arg(short, long)] pub lazy: bool,

  /// Create and configure manifest in current working folder
  #[arg(long)] pub create: bool,

  /// Clear config, cache and registry folders
  #[arg(long)] pub purge: bool,

  /// Specify for purge to clear only cache
  #[arg(long)] pub cache_only: bool,

  /// Set username for artifactory OAuth. Use --token to set token.
  #[arg(long)] pub username: Option<String>,

  /// Set token for artifactory OAuth. Use --username to set username.
  #[arg(long)] pub token: Option<String>,

  /// Push file to artifactory
  #[arg(long)] pub push: Option<String>,

  /// Specify distribution to push to artifactory
  #[arg(long)] pub distribution: Option<String>,

  /// Get specified field from manifest in current working folder
  /// Allowed values:
  ///   - name
  ///   - version
  ///   - authors
  ///   - description
  /// Example usage: poppy --manifest-info name
  /// Output: example-package
  #[arg(long, verbatim_doc_comment)] pub manifest_info: Option<String>,

  /// Gitlab-CI username
  #[arg(long)] pub ci_git_username: Option<String>,

  /// Gitlab-CI token
  #[arg(long)] pub ci_git_token: Option<String>,

  /// Returns absolute path to dependencies directory of current manifest
  /// Path string will be different for each platform (win32, linux)
  ///
  /// Example usage: poppy --install-path
  /// Output: C:\\user\\.poppy\\dependencies (Windows) or /home/user/.poppy/dependencies (Linux)
  #[arg(long, verbatim_doc_comment)] pub install_path: bool
}