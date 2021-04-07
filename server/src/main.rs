use server::TCPChatServer;

fn main() {
    let server = TCPChatServer::new();
    let server_handle = server.run();
    server_handle.join().unwrap();
}
