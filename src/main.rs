use std::net::SocketAddr;

use anyhow::anyhow;
use bytes::Bytes;
use clap::Parser;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};

use hyper_util::rt::TokioIo;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::time;
use tokio::time::Duration;
use tokio::time::Interval;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    endpoint: String,
}
#[tokio::main]
async fn main() {
    if let Err(e) = main_with_error(1).await {
        println!("{:?}", e);
    }
}
async fn main_with_error(ntype: i32) -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let endpoint = args.endpoint;
    let mut stream = TcpStream::connect(endpoint)
        .await
        .map_err(|e| anyhow!("Connect error:{}", e))?;
    println!("created stream");
    let mut interval = time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        let con_command: Vec<u8> = match ntype {
            1 => vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x01, 0x03, 0x00, 0x00, 0x00, 0x02,
            ],
            _ => vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x11, 0x03, 0x00, 0x60, 0x00, 0x01,
            ],
        };
        stream
            .write_all(&con_command)
            .await
            .map_err(|e| anyhow!("write all error,error is {}", e))?;
        stream
            .flush()
            .await
            .map_err(|e| anyhow!("flush error,error is {}", e))?;
        let mut res_byte_head = [0u8; 16];
        let read_res = stream
            .read_exact(&mut res_byte_head)
            .await
            .map_err(|e| anyhow!("read_exact error,error is {}", e))?;

        let body_len = res_byte_head[7] as usize;
        let mut res_body = vec![0u8; body_len];
        // parse()

        println!("resbody: {:?}", res_body);

        let mut n_weight: u16 = 0;
        let mut n_temp: u16 = 0;
        let mut n_gain: u16 = 256;

        if ntype == 1 {
            n_weight = (res_byte_head[9] as u16) * n_gain + res_byte_head[10] as u16;
            n_temp = (res_byte_head[11] as u16) * n_gain + res_byte_head[12] as u16;
        } else {
            let currnet = (((res_byte_head[9] as u16) * n_gain + res_byte_head[10] as u16) / 1000
                - 4) as f64
                * 4000.0
                / 16.0;
            n_weight = currnet as u16;
            n_temp = 0;
        }

        println!("wrote to stream; success={}", n_weight);
    }
    Ok(())
}
