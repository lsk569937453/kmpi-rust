
use anyhow::anyhow;
use clap::Parser;
use http_body_util::{BodyExt};
use service::user_service::login;
use service::user_service::{add_user, admin_login, delete_user, update_user};

mod common;
mod dao;
mod service;
mod vojo;
#[macro_use]
extern crate anyhow;
use crate::common::init::init_with_error;
use crate::service::vessl_service::get_vessl;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time;
use tokio::time::Duration;
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_appender::rolling;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    endpoint: String,
}
use axum::{
    routing::{get, post}, Router,
};
use serde::{Deserialize};
fn setup_logger() -> Result<WorkerGuard, anyhow::Error> {
    let app_file = rolling::daily("./logs", "access.log");
    let (non_blocking_appender, guard) = NonBlockingBuilder::default()
        .buffered_lines_limit(10)
        .finish(app_file);
    let file_layer = tracing_subscriber::fmt::Layer::new()
        .with_target(true)
        .with_ansi(false)
        .with_writer(non_blocking_appender)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(tracing_subscriber::filter::LevelFilter::TRACE)
        .init();
    Ok(guard)
}
#[tokio::main]
async fn main() {
    if let Err(e) = main_with_error().await {
        println!("{:?}", e);
    }
}
async fn main_with_error() -> Result<(), anyhow::Error> {
    let _work_guard = setup_logger()?;
    let db_pool = common::sql_connections::create_pool().await?;
    init_with_error(db_pool.clone()).await?;
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/vessel", get(get_vessl))
        .route("/api/adminLogin", post(admin_login))
        .route("/api/login", post(login))
        .route("/api/deleteUser", post(delete_user))
        .route("/api/addUser", post(add_user))
        .route("/api/updateUser", post(update_user))
        .with_state(db_pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

async fn another_with_error(ntype: i32) -> Result<(), anyhow::Error> {
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
