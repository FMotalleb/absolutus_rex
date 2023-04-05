use absolutus_rex::TcpProxy;
use clap::Parser;
use std::{
    fmt::{self},
    io::{self, Write},
};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Absolutus Rex is a command-line tool for port forwarding.\n\nThis tool can be used to forward ports, and may be particularly useful when used with `supervisord`."
)]
struct Args {
    /// Local port - requires a local port that is available for use.
    #[arg(short, long, default_value_t = 8990)]
    port: u16,

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
    let args = Args::parse();
    let remote = fmt::format(format_args!("{}:{}", args.remote_address, args.remote_port));
    println!("Application ran with config:");
    println!("\tPort: `{}`", args.port);
    println!("\tIs Local Only: {}!", args.local_only);
    println!("\tRemote Address: `{}`", remote);
    match TcpProxy::new(args.port, remote.parse().unwrap(), args.local_only == false) {
        Ok(_proxy) => {
            println!("Proxy State: OK!");
        }
        e => {
            println!("Proxy State: ERROR!");
            println!("Reason: {}", e.err().unwrap());
        }
    }
    let result = io::stdout().flush();
    if {
        let ref this = result;
        matches!(*this, Ok(_))
    } {
        print!("Flushed");
    }
    loop {}
}
