use std::sync::{Arc, Mutex};

use tokio::{io::{AsyncReadExt, Error}, net::{TcpListener, TcpStream, ToSocketAddrs, tcp::{OwnedWriteHalf}}, task};

type UserStreams = Arc<Mutex<Vec<OwnedWriteHalf>>>;
pub struct TCPChatServer {
    users: UserStreams,
}

impl Default for TCPChatServer {
    fn default() -> Self {
        Self::new()
    }
}

impl TCPChatServer {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(vec![]))
        }
    }
    async fn process_socket(socket: TcpStream, users: UserStreams) {
        let (read_half, write_half) = socket.into_split();
        users.lock().expect("Failed to lock an user streams storage").push(
            write_half
        );
    }
    pub async fn run<'a, T>(&mut self, addr: &'a T) -> Result<(), Error>
        where &'a T: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addr).await?;
        loop {
            let (socket, _) = listener.accept().await?;
            let users = self.users.clone();

            task::spawn(async move {
                TCPChatServer::process_socket(socket, users).await;
            });
        }
    }
}