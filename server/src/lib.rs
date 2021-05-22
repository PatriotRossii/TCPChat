use shared::ChatMessage;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, ToSocketAddrs},
    sync::broadcast,
};
use tokio_util::codec::{Decoder, Encoder};

pub struct TCPChatServer {}

impl Default for TCPChatServer {
    fn default() -> Self {
        Self::new()
    }
}

impl TCPChatServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run<T>(self, addr: T) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        T: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addr).await?;
        let (tx, _) = broadcast::channel::<ChatMessage>(100);

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let (read, write) = socket.into_split();

            let tx = tx.clone();
            let mut rx = tx.subscribe();

            tokio::spawn(async move {
                let mut socket = read;
                let mut decoder = tokio_serde_cbor::Decoder::<ChatMessage>::new();

                loop {
                    let mut buffer = bytes::BytesMut::with_capacity(1024);
                    let data = socket.read_buf(&mut buffer).await;

                    if data.is_err() {
                        println!("Err reading buffer");
                    }

                    if let Ok(Some(e)) = decoder.decode(&mut buffer) {
                        if tx.send(e).is_err() {
                            println!("Failed to send message to channel");
                        }
                    }
                }
            });

            tokio::spawn(async move {
                let mut socket = write;
                let mut encoder = tokio_serde_cbor::Encoder::<ChatMessage>::new();

                loop {
                    while let Ok(e) = rx.recv().await {
                        let mut buffer = bytes::BytesMut::with_capacity(1024);
                        if encoder.encode(e, &mut buffer).is_err() {
                            println!("Error encoding buffer");
                        }
                        if socket.write_buf(&mut buffer).await.is_err() {
                            println!("Error sending buffer");
                        }
                    }
                }
            });
        }
    }
}
