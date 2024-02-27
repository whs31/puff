use log::error;

mod consts;
mod poppy;
mod utils;

fn main() {
    utils::cli::init_cli_logger("trace")
        .map_err(|e| eprintln!("failed to init cli logger. this is critical error and should never happen."))
        .map_err(|_| std::process::exit(1) )
        .unwrap();
    poppy::run_poppy()
        .map_err(|e| { error!("fatal error: {}", e); std::process::exit(1); } )
        .unwrap();
}
