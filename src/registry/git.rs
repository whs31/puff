use colored::Colorize;
use git2_credentials::CredentialHandler;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use log::{debug, trace, warn};

pub fn clone_repository(url: &str, target_path: &str, __branch: &str, username: Option<String>, token: Option<String>) -> anyhow::Result<()>
{
  trace!("cloning git repository from {} to {}", url, target_path);

  if std::path::Path::new(target_path).exists() {
    debug!("cleaning up old repo");
    std::fs::remove_dir_all(target_path)?;
  }

  let url = if !url.ends_with(".git") {
    warn!("{} does not end with {}, adding it", ".git".red(), url);
    format!("{}.git", url)
  } else {
    url.to_string()
  };
  let mut cb = git2::RemoteCallbacks::new();
  let git_config = git2::Config::open_default()?;
  let no_fo = &username.is_some();
  let mut ch = CredentialHandler::new(git_config);
  cb.credentials(
    move |url, username, allowed| ch.try_next_credential(url, username, allowed)
  );
  let clone_pb = ProgressBar::new(100);
  clone_pb.set_style(ProgressStyle::with_template(
      "{wide_msg} {spinner:.green} {bar:30.cyan/white} {human_pos:4}/ {human_len:4} ({percent:3}%)"
    )
    .expect("setting progress bar style should not fail!")
    .progress_chars("▃ ")
  );
  clone_pb.set_draw_target(ProgressDrawTarget::stdout_with_hz(5));
  clone_pb.set_message("cloning git repository...");
  let mut clone_pb_ff = false;

  cb.transfer_progress(move |stats| {
    if stats.received_objects() == stats.total_objects() {
      if !clone_pb_ff {
        clone_pb.finish_with_message("cloning git repository done!");
        clone_pb_ff = true;
      }
    } else {
      clone_pb.set_length(stats.total_objects() as u64);
      clone_pb.set_position(stats.received_objects() as u64);
    }
    true
  });
  let mut fo = git2::FetchOptions::new();
  fo
    .remote_callbacks(cb)
    .download_tags(git2::AutotagOption::All)
    .update_fetchhead(true);
  std::fs::create_dir_all(target_path)?;

  let url_proto = url.split("://").collect::<Vec<&str>>();
  let url_proto = format!("{}://", url_proto[0]);
  let url_rest = url.split(&url_proto).collect::<Vec<&str>>()[1];
  let url_fixed = if let Some(username) = username {
    trace!("git username provided: {}", username.yellow());
    if let Some(token) = token {
      trace!("git token provided: ***{}***", token[3..token.len()-3].to_string().yellow());
      format!("{}{}:{}@{}", url_proto, username, token, url_rest)
    } else {
      format!("{}{}@{}", url_proto, username, url_rest)
    }
  } else {
    url.to_string()
  };

  trace!("finalized url: {url_fixed}");

  if no_fo.clone() {
    debug!("cloning via cmd line");
    let output = std::process::Command::new("git")
      .arg("clone")
      .arg(url_fixed.clone())
      .arg(target_path)
      .output()?;
    if !output.status.success() {
      return Err(anyhow::anyhow!("failed to clone repository: {}", String::from_utf8_lossy(&output.stderr)));
    }
  } else {
    git2::build::RepoBuilder::new()
      //.branch(branch)
      .fetch_options(fo)
      .clone(url_fixed.clone().as_str(), target_path.as_ref())?;
  }
  Ok(())
}