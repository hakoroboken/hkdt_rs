use hkdt_rs::connection::web::WebServer;
use hkdt_rs::{log_info, log_warn};

#[tokio::main]
async fn main() {
    log_info!("TCPサーバーを起動します...");
    let mut server = WebServer::new("MyTCPServer");

    if server.open_server("192.168.11.12", 64201).await {
        log_info!("TCPサーバーが正常に起動しました。クライアントからの接続を待っています...");

        server.accept_connections().await;
    } else {
        log_info!("TCPサーバーの起動に失敗しました。");
    }

    loop {
        match server.get_recv_data().await {
            Some(data) => {
                log_info!("受信：{}", data);
            }
            None => {
                log_warn!("情報なし");
            }
        }
    }
}
