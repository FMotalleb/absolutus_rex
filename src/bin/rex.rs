use absolutus_rex::tcp_proxy::TcpProxy;
use clap::Parser;
use log::{error, info, LevelFilter};
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
    #[arg(long,default_value_t=String::from("info"))]
    log_level: String,
}
fn main() {
    let args = Args::parse();
    let log_level = parse_log_level(args.log_level);

    env_logger::Builder::new()
        .filter( Option::None,log_level)
        .format_timestamp_secs()
        .format_target(false)
        .format_indent(Option::Some(8))
        .init();
    let level = log_level.as_str();

    let remote = fmt::format(format_args!("{}:{}", args.remote_address, args.remote_port));

    info!(
        "Application ran with config:\nIs Local Only: {}!\nRemote Address: `{}`\nLOG_LEVEL: {}",
        args.local_only, remote, level
    );

    for port in args.ports {
        let remote_address = remote.clone().parse().unwrap();
        let name = fmt::format(format_args!("Thread Spawner of:{}", remote,));
        match Builder::new().name(name).spawn(move || {
            match TcpProxy::new(port, remote_address, args.local_only,false) {
                Ok(_proxy) => {
                    info!("\tPort: {},Proxy State: OK!", port);
                }
                Err(e) => {
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

fn parse_log_level(arg: String) -> LevelFilter {
    let arg = arg;
    if arg == "error"  {
        return LevelFilter::Error;
    }
    else if arg == "debug" {
        return LevelFilter::Debug;
    }else if arg == "info" {
        return LevelFilter::Info;
    }else if arg=="trace" {
        return LevelFilter::Trace;
    }else if arg=="off"{
        return LevelFilter::Off;
    }else if arg=="warn"{
        return LevelFilter::Warn;
    }
    println!("debug level is invalid using `error` instead.");
    LevelFilter::Error
    // println!("debug level is invalid using `error` instead.");
    // "error".to_string()
}
