use clap::Parser;
use front_end::app_api_call;
use std::{env, path::PathBuf};
#[cfg(feature = "tracing")]
use tracing::info;
#[cfg(feature = "tracing")]

fn default_log_path() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    println!("Current path: {:?}", path);
    path.pop();
    path.push("log/debug.log");
    println!("Default log path: {:?}", path);
    path
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long("ip"), default_value = "[::1]")]
    ip_address: String,

    #[arg(default_value = default_log_path().into_os_string())]
    log_path: PathBuf,
}

const PNAME: &str = env!("CARGO_PKG_NAME");
const PVSN: &str = env!("CARGO_PKG_VERSION_PATCH");

pub fn main() {
    // Argument parsing
    let cli = Cli::parse();

    // Global subscription to tracing events
    tracing_subscriber::fmt::init();

    // Demo functionality
    info!(PNAME, "Info call with package name");
    println!("Default CLI argument: IP Address = {}", cli.ip_address);
    let some_data: u32 = PVSN.parse().unwrap();
    let data_returned = app_api_call(some_data);
    info!(
        data_modified = data_returned != some_data,
        data_returned, "Data processing complete"
    )
}
