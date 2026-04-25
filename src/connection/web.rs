use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;

use crate::connection::thread::{MessageSender, ThreadConnection};
use crate::{log_err, log_info};

const THREAD_MESSAGE_ERROR: &str = "ERRRRRRRRRRRRR";

pub struct WebServer {
    name: String,
    listener: Option<TcpListener>,
    th_connection: ThreadConnection<String>,
}

impl WebServer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            listener: None,
            th_connection: ThreadConnection::new(),
        }
    }

    pub async fn open_server(&mut self, addr: &str, port: u16) -> bool {
        if self.listener.is_some() {
            return false;
        }

        let address = format!("{}:{}", addr, port);

        match TcpListener::bind(address.as_str()).await {
            Ok(listener) => {
                self.listener = Some(listener);
                true
            }
            Err(_e) => {
                log_err!(
                    "[{}] サーバーの開始に失敗しました on {}",
                    self.name,
                    address
                );
                return false;
            }
        }
    }

    pub async fn accept_connections(&mut self) {
        if let Some(listener) = &self.listener {
            if let Ok((stream, addr)) = listener.accept().await {
                let ws_stream = accept_async(stream).await.unwrap();
                log_info!("[{}] クライアントが接続しました: {}", self.name, addr);
                Self::stream_loop(
                    self.th_connection.get_sender(),
                    self.name.clone(),
                    ws_stream,
                );
            }
        }
    }

    pub async fn get_recv_data(&mut self) -> Option<String> {
        self.th_connection.recv_data().await
    }

    fn stream_loop(
        message_sender: MessageSender<String>,
        name: String,
        stream: tokio_tungstenite::WebSocketStream<TcpStream>,
    ) {
        tokio::spawn(async move {
            log_info!("[{}] WebSocket接続を確立", name);
            let (_, mut read) = stream.split();

            while let Some(msg) = read.next().await {
                let msg = match msg {
                    Ok(m) => m,
                    Err(e) => {
                        log_err!("[{}] メッセージエラー : {}", name, e);
                        break;
                    }
                };

                if msg.is_text() {
                    let text = msg.to_text().unwrap();

                    let _ = message_sender.send(text.to_string()).await;
                    // log_info!("{}", text);
                } else if msg.is_close() {
                    log_err!("[{}] クライアントが切断されました", name);
                    let _ = message_sender.send(THREAD_MESSAGE_ERROR.to_string()).await;
                    break;
                }
            }
        });
    }
}
