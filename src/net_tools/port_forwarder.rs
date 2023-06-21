use std::{io};
use std::net::{SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::io::{Error, ErrorKind};

use crate::net_tools::config::Config;

pub async fn exec(cfg: Config) -> io::Result<()> {
    let listener = TcpListener::bind(cfg.local_ep).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let peer_addr = socket.peer_addr().unwrap().to_string();
        tokio::spawn(async move {
            match process(socket, cfg.remote_ep).await {
                Ok(_) => {},
                Err(err) => println!("Connection from {} to {} closed [{}]", peer_addr, cfg.remote_ep, err),
            }
        });
    };
}

async fn process(mut local_stream: TcpStream, remote_addr: SocketAddr) -> Result<String, io::Error> {
    let mut remote_stream = TcpStream::connect(remote_addr).await.unwrap();
    println!("Connection from {} to {remote_addr} established", local_stream.peer_addr().unwrap().to_string());

    let (mut lrd, mut lwd) = local_stream.split();
    let (mut rrd, mut rwd) = remote_stream.split();

    let mut src_buf = vec![0; 2048];
    let mut dst_buf = vec![0; 2048];

    loop {
        tokio::select! {
            res = async {
                let n = lrd.read(&mut src_buf).await?;
                if n == 0 {
                    return Err(Error::new(ErrorKind::Other, "Ok"))
                }
                rwd.write_all(&src_buf[..n]).await?;
                Ok::<_, io::Error>(())
            } => {res?}
            res = async {
                let n = rrd.read(&mut dst_buf).await?;
                if n == 0 {
                    return Err(Error::new(ErrorKind::Other, "Ok"))
                }
                lwd.write_all(&dst_buf[..n]).await?;
                Ok::<_, io::Error>(())
            } => {res?}
            else => {}
        }
    }
}
