use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use toml::Value;

use crate::client_state::ClientState;
use crate::commands::CommandsEval;
use crate::memory::MemoryDb;

pub struct Redust {
    pub tcp_listener: TcpListener,
    pub eval: Arc<Mutex<CommandsEval>>,
    pub data: Arc<Mutex<MemoryDb>>,
}

impl Redust {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let tcp_listener = TcpListener::bind(addr).await?;

        let data = Arc::new(Mutex::new(MemoryDb::new()));

        let config = std::fs::read_to_string("config.toml")?;
        let config: Value = toml::from_str(&config)?;

        let admin_password = config
            .get("password")
            .and_then(|value| value.as_str())
            .unwrap_or("password")
            .to_owned();

        let eval = Arc::new(Mutex::new(CommandsEval {
            admin_password,
            database: data.clone(),
        }));

        Ok(Self {
            tcp_listener,
            data,
            eval,
        })
    }

    pub async fn run(&self) {
        loop {
            let (socket, _) = self.tcp_listener.accept().await.unwrap();

            let eval = self.eval.clone();

            tokio::spawn(async move {
                Redust::handle_connection(&eval, socket).await;
            });
        }
    }

    pub async fn handle_connection(
        eval: &Arc<Mutex<CommandsEval>>,
        mut socket: tokio::net::TcpStream,
    ) {
        let mut buf = [0; 2048];
        let mut str_buffer = String::new();

        let mut client_state = ClientState { auth: false };

        loop {
            let n = match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            let s = std::str::from_utf8(&buf[0..n]);
            str_buffer.push_str(s.unwrap());

            if s.is_err() {
                eprintln!("failed to parse buffer");
                return;
            }

            let p = s.unwrap().find(';');

            if p.is_none() {
                str_buffer.push_str(s.unwrap());
                continue;
            }

            let new_buffer = str_buffer.split_off(p.unwrap() + 1);

            let result = eval.lock().unwrap().eval(&str_buffer, &mut client_state);

            str_buffer = new_buffer;

            if let Err(e) = socket.write_all(result.as_bytes()).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return;
            }
        }
    }
}
