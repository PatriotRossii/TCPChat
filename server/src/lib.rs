use std::io::BufRead;
use std::{
    io::{BufReader, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

pub struct TCPChatServer {
    storage: Arc<Mutex<Vec<TcpStream>>>,
}

impl Default for TCPChatServer {
    fn default() -> Self {
        Self::new()
    }
}

impl TCPChatServer {
    pub fn new() -> Self {
        let storage = Arc::new(Mutex::new(vec![]));
        TCPChatServer { storage }
    }

    pub fn run(&self) -> JoinHandle<()> {
        let handle_storage = self.storage.clone();
        thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:80").unwrap();
            println!("Start listening on {:?}", &listener);

            let (sender, receiver) = mpsc::channel::<String>();

            let translator_storage = handle_storage.clone();
            thread::spawn(move || loop {
                if let Ok(message) = receiver.recv() {
                    for mut client in translator_storage.lock().unwrap().iter() {
                        println!("Transmitting {}", &message);
                        client.write_all(message.as_bytes()).unwrap();
                    }
                }
            });

            for stream in listener.incoming() {
                let stream = stream.unwrap();
                stream.set_read_timeout(None).unwrap();

                let stream_addr = stream.local_addr().unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());

                println!("Successfully established connection with {:?}", stream_addr);

                handle_storage
                    .lock()
                    .unwrap()
                    .push(stream.try_clone().unwrap());
                let sender = sender.clone();

                thread::spawn(move || loop {
                    println!("Trying to read message from {:?}", &stream_addr);

                    let mut buf = String::new();
                    let res = reader.read_line(&mut buf);
                    if res.is_ok() {
                        println!(
                            "Successfully read message from {:?}. Transmitting",
                            &stream_addr
                        );
                        match sender.send(buf) {
                            Ok(_) => println!("Message was successfully transmitted"),
                            Err(e) => println!("Message was failed to transmit because of {:?}", e),
                        }
                    } else {
                        println!(
                            "Reading message from {} was failed. Shutting down the connection",
                            &stream_addr
                        );
                        stream.shutdown(Shutdown::Both).expect("Failed to shutdown");
                        println!("Connection was successfully shutted down");
                        break;
                    }
                });
            }
        })
    }
}
