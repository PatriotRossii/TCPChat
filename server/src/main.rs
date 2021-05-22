use server::TCPChatServer;

#[tokio::main]
pub async fn main() {
    let addr = std::env::var("ADDR").expect("Please, specify env var: ADDR");
    let server = TCPChatServer::new();

    if let Err(err) = server.run(addr).await {
        println!("Error: {:?}", err);
    }
}
