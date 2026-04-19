// このExampleはArmBotのセンサーデータをシリアル通信を介して取得します。

// シリアル通信用ライブラリ
use hkdt_rs::connection::serial::Serial;
// ArmBotを処理するためのライブラリ
use hkdt_rs::arm_bot::ArmBot;
// デバッグ出力用のマクロ
use hkdt_rs::{log_info, log_err};

fn main()
{
    // シリアルポートを開く。デバイス名とボーレートは環境に合わせて変更してください。
    let mut serial = Serial::new("/dev/ttyACM0", 115200).unwrap();

    // ArmBotのインスタンスを作成
    let mut arm_bot = ArmBot::new();
    
    // シリアル通信による受信、そしてセンサーデータの更新をループで行います。
    loop {
        // シリアルポートからデータを読み取ります。
        let from_arm_bot = serial.read_str();
        
        // 読み取ったデータはOption型で返されるため、Some(読み取り成功)かNone(読み取り失敗)かを確認します。
        match from_arm_bot {
            Some(line) =>{
                // 読み取ったデータをArmBotのセンサーデータとして更新します。
                arm_bot.update_sensor(line);
                // 更新されたセンサーデータをログに出力します。
                log_info!("Hand : {:?}", arm_bot.get_hand_motor());
            },
            None => {
                log_err!("データが空でした。シリアル通信に問題がある可能性があります。");
            },
        }
    }
}