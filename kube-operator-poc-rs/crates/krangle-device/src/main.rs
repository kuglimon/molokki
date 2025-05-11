use tokio::{io::AsyncWriteExt, sync::oneshot};
use tracing::info;

pub struct Device {
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl Device {
    pub async fn start(ip: &str, port: u16) -> Self {
        info!("Device starting");
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let addr = format!("{ip}:{port}");
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        let _ = tokio::spawn(async move {
            tokio::select! {
                _ = async {
                    loop {
                        let (mut socket, _) = listener.accept().await.unwrap();
                        info!("Device accepted connection");
                        let _  =  socket.shutdown();
                        info!("Device closing connection");
                    }
                } => {},
                _ = shutdown_rx => {
                    info!(%addr, "Device shutting down");
                }
            }
        })
        .await;

        Self {
            shutdown_tx: Some(shutdown_tx),
        }
    }

    pub async fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

#[tokio::main]
async fn main() {
    // Set up logging
    tracing_subscriber::fmt::init();

    let _ = Device::start("127.0.0.1", 8080).await;
}
