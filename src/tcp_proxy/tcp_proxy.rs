use log::{debug, error};
use std::fmt;
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
pub struct TcpProxy {
    pub forward_thread: std::thread::JoinHandle<()>,
}

impl TcpProxy {
    pub fn new(
        listen_port: u16,
        proxy_to: SocketAddr,
        local_only: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ip = if local_only {
            Ipv4Addr::LOCALHOST
        } else {
            Ipv4Addr::UNSPECIFIED
        };
        let listener_forward = TcpListener::bind(SocketAddr::new(IpAddr::V4(ip), listen_port))?;
        let name = fmt::format(format_args!("Listener thread of port:{}", listen_port,));
        let forward_thread = match std::thread::Builder::new().name(name).spawn(move || {
            loop {
                let (stream_forward, _addr) = listener_forward
                    .accept()
                    .expect("Failed to accept connection");
                debug!("New connection");

                let mut sender_forward = TcpStream::connect(proxy_to).expect("Failed to bind");
                let sender_backward = sender_forward.try_clone().expect("Failed to clone stream");
                let mut stream_backward =
                    stream_forward.try_clone().expect("Failed to clone stream");
                let name = fmt::format(format_args!("Forward Stream of port:{}", listen_port,));
                match std::thread::Builder::new().name(name).spawn(move || {
                    let mut stream_forward = BufReader::new(stream_forward);
                    loop {
                        let length = {
                            let buffer = stream_forward.fill_buf();

                            let buffer = buffer.unwrap_or_default();
                            let length = buffer.len();
                            if buffer.is_empty() {
                                // Connection closed
                                debug!("Client closed connection");
                                return;
                            }

                            sender_forward
                                .write_all(buffer)
                                .expect("Failed to write to remote");
                            sender_forward.flush().expect("Failed to flush remote");
                            length
                        };
                        stream_forward.consume(length);
                    }
                }) {
                    Ok(_proxy) => {}
                    Err(e) => {
                        error!("forward stream crashed: {}", e);
                    }
                };
                let name = fmt::format(format_args!("Backward Stream of port:{}", listen_port,));
                let _backward_thread =
                    match std::thread::Builder::new().name(name).spawn(move || {
                        let mut sender_backward = BufReader::new(sender_backward);
                        loop {
                            let length = {
                                let buffer = sender_backward.fill_buf().unwrap_or_default();
                                let length = buffer.len();
                                if buffer.is_empty() {
                                    debug!("Remote closed connection");
                                    return;
                                }
                                if stream_backward.write_all(buffer).is_err() {
                                    debug!("Client closed connection");
                                    return;
                                }

                                stream_backward.flush().expect("Failed to flush locally");
                                length
                            };
                            sender_backward.consume(length);
                        }
                    }) {
                        Ok(_proxy) => {}
                        Err(e) => {
                            error!("forward stream crashed: {}", e);
                        }
                    };
            }
        }) {
            Ok(handle) => handle,
            Err(e) => {
                error!("Thread Spawner crashed: {}", e);
                return Err(Box::new(e) as Box<dyn std::error::Error>);
            }
        };

        Ok(Self { forward_thread })
    }
}
