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
    let mut interval = time::interval(Duration::from_secs(2));
    loop {
        interval.tick().await;
        let write_command: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00b, 0x02, 0x10, 0x00, 0x21, 0x00, 0x02, 0x04, 0x00,
            0x00, 0x00, 0x00,
        ];
        let read_command: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x02, 0x03, 0x00, 0x05, 0x00, 0x04,
        ];
        let write_btn_enable: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x02, 0x06, 0x00, 0x17, 0x00, 0x01,
        ];
        // 发送数据
        stream.write_all(&read_command).await?;
        let _ = stream.flush().await;
        //fmt.Println(writeLen)
        let mut res_byte_head = vec![0; 32];
        let _ = stream.read(&mut res_byte_head).await?;
        // let weight = (res_byte_head[9] as u16 * 256 + res_byte_head[10] as u16) as i32;
        // let ok = res_byte_head[12] as i32;
        // let n_fill_al_finished = res_byte_head[14] as i32;
        // let n_press_button = (res_byte_head[16] as u16) as i32;
        println!("res_byte_head is {:?}", res_byte_head);
    }
    Ok(())
}
