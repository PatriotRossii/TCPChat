use client::TCPChatClient;
use fprint::fprint;

fn main() {
    let mut client = TCPChatClient::connect("127.0.0.1:80");
    let stdin = std::io::stdin();

    loop {
        fprint!("Select your mode (read/write): ");
        let mut mode = String::new();
        stdin.read_line(&mut mode).unwrap();

        match mode.trim() {
            "read" => loop {
                if let Ok(message) = client.read_message() {
                    fprint!("{}: {}", message.nickname, message.content);
                }
            },
            "write" => loop {
                let mut nickname = String::new();
                let mut content = String::new();

                fprint!("Enter your nickname: ");
                stdin.read_line(&mut nickname).unwrap();

                fprint!("Enter your message: ");
                stdin.read_line(&mut content).unwrap();

                let nickname = String::from(nickname.trim());
                let content = String::from(content.trim());

                match client.send_message(nickname, content) {
                    Ok(_) => println!("Message was successfully sended!"),
                    Err(_) => println!("Message was failed to send"),
                }
            },
            _ => continue,
        }
    }
}
