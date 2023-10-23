use spark_gpt::big_mod::BigMod;
use std::io;
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

fn main() {
    let big_mod = BigMod::new(
        String::from(""),
        String::from(""),
        String::from(""),
    );

    loop {
        println!("me:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取行");

        let mut auth_url = big_mod.get_auth_url();
        auth_url = auth_url.replace("http", "ws").replace("https", "wss");

        let (mut socket, response) =
            connect(Url::parse(&auth_url).expect("Can't parse")).expect("Can't connect");

        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());

        socket
            .send(Message::Text(big_mod.gen_params(input)))
            .unwrap();

        print_message_from_socket(socket);
        println!("******************************************\n");
    }
}

fn print_message_from_socket(mut socket: WebSocket<MaybeTlsStream<TcpStream>>) {
    println!("spark:");
    loop {
        let msg = socket.read().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                println!("Connect error, please retry");
                return;
            }
        };
        let data: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");

        let choices = &data["payload"]["choices"];
        if let serde_json::Value::String(content) = &choices["text"][0]["content"] {
            print!("{content}");
        };

        if let serde_json::Value::Number(status) = &choices["status"] {
            if status.as_i64() == Some(2) {
                break;
            }
        }
    }
}
