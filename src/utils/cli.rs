use flexi_logger::AdaptiveFormat;

pub fn init_cli_logger(level: &str) -> anyhow::Result<()>
{
    flexi_logger::Logger::try_with_str(level)?
        .adaptive_format_for_stdout(AdaptiveFormat::Default)
        .adaptive_format_for_stderr(AdaptiveFormat::Default)
        .start()?;
    Ok(())
}