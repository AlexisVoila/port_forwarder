use std::{env, process};
use tokio::io;
use port_forwarder::{net_tools};

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cfg = net_tools::config::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Configuration: {:?}", cfg);
    net_tools::port_forwarder::exec(cfg).await?;

    Ok(())
}

