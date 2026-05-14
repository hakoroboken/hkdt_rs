use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    signal,
};

use std::process::Stdio;

struct ProcessUnit {
    name: String,
    child: tokio::process::Child,
}

pub struct MultiProcessLauncher {
    process: Vec<ProcessUnit>,
}

impl MultiProcessLauncher {
    pub fn new() -> Self {
        MultiProcessLauncher {
            process: Vec::new(),
        }
    }

    pub async fn launch(&mut self, cmds: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        for name in cmds {
            crate::log_info!("[Launch] {}を実行", name);
            let mut child = Command::new(name)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            if let Some(stdout) = child.stdout.take() {
                tokio::spawn(async move {
                    let mut reader = BufReader::new(stdout).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        crate::log_info!("{}", line);
                    }
                });
            }

            // stderr処理
            if let Some(stderr) = child.stderr.take() {
                tokio::spawn(async move {
                    let mut reader = BufReader::new(stderr).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        crate::log_err!("{}", line);
                    }
                });
            }

            self.process.push(ProcessUnit {
                name: name.to_string(),
                child,
            });
        }

        signal::ctrl_c().await?;

        for ch in &mut self.process {
            if let Some(id) = ch.child.id() {
                crate::log_info!("プロセスを終了します :{} (pid={})", ch.name, id);
            }
            let _ = ch.child.kill().await;
        }

        Ok(())
    }
}
