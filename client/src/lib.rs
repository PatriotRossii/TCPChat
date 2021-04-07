use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpStream, ToSocketAddrs},
};

const DELIMITER: &str = "\u{6}";

#[derive(Debug)]
pub enum ChatClientErr {
    FailedToRead,
    FailedToWrite,
}

pub struct TCPChatClient {
    conn: TcpStream,
    reader: BufReader<TcpStream>,
}

pub struct Message {
    pub nickname: String,
    pub content: String,
}

impl Message {
    fn from(nickname: String, content: String) -> Self {
        Self { nickname, content }
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        format!("{}{}{}\r\n", self.nickname, DELIMITER, self.content)
    }
}

impl From<String> for Message {
    fn from(encoded: String) -> Self {
        let mut decoded = encoded.splitn(2, DELIMITER);
        Self {
            nickname: decoded.next().unwrap().into(),
            content: decoded.next().unwrap().into(),
        }
    }
}

impl TCPChatClient {
    pub fn connect<T>(addr: T) -> Self
    where
        T: ToSocketAddrs,
    {
        let connection = TcpStream::connect(addr).unwrap();
        connection.set_read_timeout(None).unwrap();

        let reader = BufReader::new(connection.try_clone().unwrap());

        Self {
            conn: connection,
            reader,
        }
    }

    pub fn send_message(&mut self, nickname: String, content: String) -> Result<(), ChatClientErr> {
        let packet = Message::from(nickname, content);
        let message_to_send = packet.to_string();
        self.conn
            .write_all(message_to_send.as_bytes())
            .map_err(|_| ChatClientErr::FailedToWrite)
            .unwrap();
        Ok(())
    }

    pub fn read_message(&mut self) -> Result<Message, ChatClientErr> {
        let mut buf = String::new();
        self.reader
            .read_line(&mut buf)
            .map_err(|_| ChatClientErr::FailedToRead)
            .unwrap();
        Ok(<Message as From<String>>::from(buf))
    }
}
