mod lib;

use std::io::stdin;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use log::debug;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::client::IntoClientRequest;
use tungstenite::http::{Response as HttpResponse};
use tungstenite::stream::MaybeTlsStream;
use url::Url;
use crate::lib::request::format_json_command;

fn ws_reader(shared_writer: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>) -> Result<Option<Message>, tungstenite::Error>  {
    match shared_writer.lock().unwrap().read() {
        Ok(msg) => {
            Ok(Some(msg))
        },
        Err(err) => match err {
            tungstenite::Error::Io(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    Ok(None)
                } else {
                    Err(tungstenite::Error::Utf8)
                }
            },
            _ => Err(tungstenite::Error::ConnectionClosed),
        },
    }
}

fn main() {
    let symbols_array = vec!["btcusdt", "ethbtc", "ethusdt"];
    let mut thread_handle_vec: Vec<JoinHandle<()>> = Vec::with_capacity(symbols_array.len());

    env_logger::init();
    // wss://testnet.binance.vision
    let web_socket_url = "wss://stream.binance.com:9443/ws/btcusdt";

    let (mut socket, response) = match connect(Url::parse(web_socket_url).unwrap()) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("err = {:?}", err);
            panic!("exit on error!")
        },
    };
    match socket.get_mut() {
        tungstenite::stream::MaybeTlsStream::Plain(s) => s.set_nonblocking(true),
        tungstenite::stream::MaybeTlsStream::NativeTls(s) => s.get_mut().set_nonblocking(true),
        _ => {
            Ok(())
        },
    }.unwrap();

    let shared_socket = Arc::new(Mutex::new(socket));

    dump_headers(&response);
    let unsub_message = format_json_command("UNSUBSCRIBE", "btcusdt", 1);

    let shared_reader = shared_socket.clone();
    thread::spawn(move || loop {
        let mut reader = ws_reader(shared_reader.clone()).unwrap();
        if reader.is_none() {
            continue;
        }
        let msg = reader.take().unwrap();
        println!("Received: {}", msg);
    });

    thread::sleep(Duration::from_millis(1000));
    for (index, symbol) in symbols_array.iter().enumerate() {
        let sub_message = format_json_command("SUBSCRIBE", symbol, index);
        let shared_writer = shared_socket.clone();
        let thread_handle = thread::spawn(move || -> () {
            shared_writer.lock().unwrap().send(Message::Text(sub_message)).unwrap();
        });
        thread_handle_vec.push(thread_handle);
    }

    thread::park();
}

fn dump_headers(response: &HttpResponse<Option<Vec<u8>>>) {
    println!("Connected to the binance server - OK!");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }
}

fn check_finish_app() -> bool {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    let control_letter = buf.trim().to_ascii_lowercase();
    if control_letter == "y" {
        debug!("entered letter: {control_letter:?}");
        return true;
    }
    false
}