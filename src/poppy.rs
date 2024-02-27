use log::{debug, error, info, trace, warn};

pub fn run_poppy() -> Result<(), anyhow::Error>
{
    trace!("i am trace");
    debug!("i am debug");
    info!("Hello, poppy!");
    warn!("asdasd");
    error!("err");
    Ok(())
}