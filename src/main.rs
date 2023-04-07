use absolutus_rex::TcpProxy;
use clap::Parser;
use log::{error, info};
use std::{fmt, thread::Builder};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Absolutus Rex is a command-line tool for port forwarding.",
    long_about = None
)]

struct Args {
    /// Local ports - requires local ports that are available for use.
    #[arg(short, long)]
    ports: Vec<u16>,

    /// This flag only opens a port on the local network (127.0.0.1), and is intended for debugging purposes.
    #[arg(short, long, default_value_t = false)]
    local_only: bool,

    /// This flag only opens a port on the local network (127.0.0.1), and is intended for debugging purposes.
    #[arg(short = 'a', long = "r-address")]
    remote_address: String,
    /// This flag only opens a port on the local network (127.0.0.1), and is intended for debugging purposes.
    #[arg(long = "r-port")]
    remote_port: u16,
    /// LogLevel
    #[arg(long)]
    log_level: Option<String>,
}
fn main() {
    let args = Args::parse();

    let log_level = args.log_level.unwrap_or("info".to_string());
    let log_level = parse_log_level(log_level);

    env_logger::Builder::from_env(
        env_logger::Env::new()
            .filter("LOG_LEVEL")
            .default_filter_or(log_level.clone()),
    )
    // .format(formatter)
    .format_timestamp_secs()
    .format_target(false)
    .format_indent(Option::Some(8))
    .init();
    let level = match std::env::var("LOG_LEVEL") {
        Ok(value) => value,
        _ => log_level,
    };

    let remote = fmt::format(format_args!("{}:{}", args.remote_address, args.remote_port));

    info!(
        "Application ran with config:\nIs Local Only: {}!\nRemote Address: `{}`\nLOG_LEVEL: {}",
        args.local_only, remote, level
    );

    for port in args.ports {
        let remote_address = remote.clone().parse().unwrap();
        let name = fmt::format(format_args!("Thread Spawner of:{}", remote,));
        match Builder::new().name(name).spawn(move || {
            match TcpProxy::new(port, remote_address, args.local_only) {
                Ok(_proxy) => {
                    info!("\tPort: {},Proxy State: OK!", port);
                }
                Err(e) => {
                    // error!("Port: {},Proxy State: ERROR!", port);
                    error!("Port: {},Reason: {}", port, e);
                }
            }
        }) {
            Ok(_proxy) => {}
            Err(e) => {
                error!("Thread Spawner crashed: {}", e);
            }
        }
    }

    std::thread::park();
}

fn parse_log_level(arg: String) -> String {
    let arg = arg.to_string();
    if arg == "error" || arg == "debug" || arg == "info" {
        return arg;
    }
    println!("debug level is invalid using `error` instead.");
    return "error".to_string();
}
