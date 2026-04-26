// このExampleはArmBotのセンサーデータをシリアル通信を介して取得します。

use hkdt_rs::arm_bot::ArmBot; // ArmBotを処理するためのライブラリ
use hkdt_rs::connection::serial::Serial; // シリアル通信用ライブラリ
use hkdt_rs::connection::udp::UdpHandler; // UDP通信用ライブラリ
use hkdt_rs::{impl_jsonable, json::Jsonable};
use hkdt_rs::{log_err, log_info, log_warn}; // デバッグ出力用のマクロ

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
struct FromServerData {
    x: f64,
    y: f64,
    rot: f64,
}
impl_jsonable!(FromServerData);

fn main() {
    // シリアルポートを開く。デバイス名とボーレートは環境に合わせて変更してください。
    let mut serial = match Serial::new("Serial", "/dev/ttyACM0", 115200) {
        Some(s) => s,
        None => {
            log_err!("シリアルポートの初期化に失敗しました。");
            log_err!("プログラムを終了します。");
            return;
        }
    };

    let mut udp = UdpHandler::new("UdpReceiver");
    udp.open_localhost(4000);

    // ArmBotのインスタンスを作成
    let mut arm_bot = ArmBot::new();

    // シリアル通信による受信、そしてセンサーデータの更新をループで行います。
    loop {
        match udp.recv() {
            Some(str) => {
                let data = FromServerData::from_string(str.as_str());
            }
            None => {}
        }

        let send_data = arm_bot.create_send_buffer(0.0, 0.0, 0.0, 0, 0, 0);
        let write_result = serial.write(&send_data);

        if write_result {
            log_info!("データを送信しました: {:?}", send_data);
        } else {
            log_err!("データの送信に失敗しました。");
        }

        // シリアルポートからデータを読み取ります。
        let from_arm_bot = serial.read_str();

        // 読み取ったデータはOption型で返されるため、Some(読み取り成功)かNone(読み取り失敗)かを確認します。
        match from_arm_bot {
            Some(line) => {
                // 読み取ったデータをArmBotのセンサーデータとして更新します。
                arm_bot.update_sensor(line);
                // 更新されたセンサーデータをログに出力します。
                log_info!("ハンド : {}", arm_bot.get_hand_motor().position);
                log_warn!("上昇機構: {}", arm_bot.get_vertical_motor().position);
                log_err!("水平移動機構: {}", arm_bot.get_horizontal_motor().position);
            }
            None => {
                log_err!("データが空でした。シリアル通信に問題がある可能性があります。");
            }
        }
    }
}
