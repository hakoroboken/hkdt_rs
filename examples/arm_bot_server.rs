use hkdt_rs::connection::web::WebServer;
use hkdt_rs::{impl_jsonable, json::Jsonable};
use hkdt_rs::{log_info, log_warn};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
struct FromServerData {
    x: f64,
    y: f64,
    rot: f64,
}
impl_jsonable!(FromServerData);

#[tokio::main]
async fn main() {
    log_info!("TCPサーバーを起動します...");
    let mut server = WebServer::new("ArmBotServer");

    if server.open_server("192.168.11.12", 64201).await {
        log_info!("TCPサーバーが正常に起動しました。クライアントからの接続を待っています...");

        server.accept_connections().await;
    } else {
        log_info!("TCPサーバーの起動に失敗しました。");
    }

    loop {
        match server.get_recv_data().await {
            Some(str) => {
                let data = FromServerData::from_string(&str);

                log_info!("x : {}, y : {}, rot : {}", data.x, data.y, data.rot);
            }
            None => {
                log_warn!("情報なし");
            }
        }
    }
}
