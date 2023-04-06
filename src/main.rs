use absolutus_rex::TcpProxy;
use clap::Parser;
use log::{debug, error, set_max_level, LevelFilter};
use std::fmt;
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Absolutus Rex is a command-line tool for port forwarding.\n\nThis tool can be used to forward ports, and may be particularly useful when used with `supervisord`."
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
}
fn main() {
    pretty_env_logger::init_custom_env("LOG_LEVEL");
    let level = match std::env::var("LOG_LEVEL") {
        Ok(value) => value,
        _ => "error".into(),
    };

    let args = Args::parse();
    let remote = fmt::format(format_args!("{}:{}", args.remote_address, args.remote_port));
    println!("Application ran with config:");
    println!("\tIs Local Only: {}!", args.local_only);
    println!("\tRemote Address: `{}`", remote);
    println!("\tLOG_LEVEL: {}", level);
    for port in args.ports {
        match TcpProxy::new(port, remote.parse().unwrap(), args.local_only) {
            Ok(_proxy) => {
                println!("\tPort: {},Proxy State: OK!", port);
            }
            e => {
                println!("Port: {},Proxy State: ERROR!", port);
                println!("Reason: {}", e.err().unwrap());
            }
        }
    }

    std::thread::park();
}
