use hkdt_rs::connection::udp::UdpHandler;
use hkdt_rs::connection::web::WebServer;
use hkdt_rs::{impl_jsonable, json::Jsonable};
use hkdt_rs::{log_err, log_info, log_warn};

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

    if server.open_server("10.181.15.175", 64201).await {
        log_info!("TCPサーバーが正常に起動しました。クライアントからの接続を待っています...");

        server.accept_connections().await;
    } else {
        log_info!("TCPサーバーの起動に失敗しました。");
    }

    log_info!("UDPソケットを作成します...");
    let mut udp = UdpHandler::new("UdpSender");

    //ローカルホスト(127.0.0.1:3500)に設定
    let open_status = udp.open_localhost(3500);
    if !open_status {
        log_err!("UDPソケットの作成に失敗しました");
        return;
    }

    // 送信相手のアドレスを設定
    udp.set_destination("127.0.0.1:4000");

    // 送信周期を20msに設定
    udp.set_send_period(20);

    loop {
        match server.get_recv_data().await {
            Some(str) => {
                let data = FromServerData::from_string(&str);

                log_info!("x : {}, y : {}, rot : {}", data.x, data.y, data.rot);

                // Stringをバイト変換して送信
                udp.send(str.as_bytes());
            }
            None => {
                log_warn!("情報なし");
            }
        }
    }
}
