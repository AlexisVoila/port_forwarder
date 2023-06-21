use std::net::{SocketAddr};
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub local_ep: SocketAddr,
    pub remote_ep: SocketAddr,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("not enough arguments");
        }

        let mut splitted = args[1].split(':');

        let local_port = match splitted.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a local port argument"),
        };

        let remote_ip = match splitted.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a remote ip address argument"),
        };

        let remote_port = match splitted.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a local port argument"),
        };

        let local_ep = SocketAddr::from_str(["0.0.0.0", local_port].join(":").as_str()).unwrap();
        let remote_ep = SocketAddr::from_str([remote_ip, remote_port].join(":").as_str()).unwrap();

        Ok(Config { local_ep, remote_ep })
    }
}
