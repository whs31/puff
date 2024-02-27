use colored::Colorize;
use git2_credentials::CredentialHandler;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use log::{debug, trace, warn};

pub fn clone_repository(url: &str, target_path: &str, __branch: &str) -> anyhow::Result<()>
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
  git2::build::RepoBuilder::new()
    //.branch(branch)
    .fetch_options(fo)
    .clone(url.as_str(), target_path.as_ref())?;
  Ok(())
}