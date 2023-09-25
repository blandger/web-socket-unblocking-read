mod lib;

use crate::lib::request::{format_binance_json_command, format_kucoin_json_command};
use crate::lib::constant::{BINANCE_WEB_SOCKET_URL, KUCOIN_PUBLIC_PATH, KUCOIN_REST_API_URL};
use crate::lib::response::KucoinPublicTokenResponse;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tungstenite::http::Response as HttpResponse;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

fn unblocking_read_message(
    shared_writer: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
) -> Result<Option<Message>, tungstenite::Error> {
    match shared_writer.lock().unwrap().read() {
        Ok(msg) => Ok(Some(msg)),
        Err(err) => match err {
            tungstenite::Error::Io(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    Ok(None)
                } else {
                    Err(tungstenite::Error::Utf8)
                }
            }
            _ => Err(tungstenite::Error::ConnectionClosed),
        },
    }
}

fn main() {
    #[allow(dead_code)]
    let symbols_array = vec!["btc-usdt", "eth-btc", "eth-usdt"];
    let mut thread_handle_vec: Vec<JoinHandle<()>> = Vec::with_capacity(symbols_array.len());

    env_logger::init();
    // prepare kucoin rest api url
    let mut rest_api = String::from(KUCOIN_REST_API_URL);
    rest_api.push_str(&KUCOIN_PUBLIC_PATH);

    // wss://testnet.binance.vision
    let web_socket_url = BINANCE_WEB_SOCKET_URL; // binance case
    // let mut web_socket_url = rest_api.trim(); // kucoin case

/*    if web_socket_url.contains("kucoin") {
        // do rest api call first to get final WS url
        let rest_api_response: KucoinPublicTokenResponse = ureq::post(&web_socket_url)
            .call().expect("Error calling rest api")
            .into_json().unwrap();
        println!("kucoin rest api response = {:?}", &rest_api_response);
        let token = rest_api_response.data.token.clone();
        let ws_end_point = rest_api_response.data.instance_servers.get(0).unwrap().endpoint.clone();
        web_socket_url = create_ws_url(token, ws_end_point).as_str();
    }*/

    let (mut socket, response) = match connect(Url::parse(web_socket_url).unwrap()) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("err = {:?}", err);
            panic!("exit on error!")
        }
    };

    // setting tungstenite's internal tcp-stream into unblocking mode
    match socket.get_mut() {
        tungstenite::stream::MaybeTlsStream::Plain(s) => s.set_nonblocking(true),
        tungstenite::stream::MaybeTlsStream::NativeTls(s) => s.get_mut().set_nonblocking(true),
        _ => Ok(()),
    }
    .unwrap();

    let shared_socket = Arc::new(Mutex::new(socket));

    dump_headers(&response);
    // let unsub_message = format_json_command("UNSUBSCRIBE", "btcusdt", 1);

    let shared_reader = Arc::clone(&shared_socket);
    thread::spawn(move || loop {
        let mut reader = unblocking_read_message(shared_reader.clone()).unwrap();
        if reader.is_none() {
            continue;
        }
        let msg = reader.take().unwrap();
        println!("WS Received: {}", msg);
    });

    thread::sleep(Duration::from_millis(1000));
    // connect to several symbols at binance
    for (index, symbol) in symbols_array.iter().enumerate() {

        let subscribe_message;
        if web_socket_url.contains("kucoin") {
            subscribe_message = format_kucoin_json_command("SUBSCRIBE", symbol, index);
        } else {
            subscribe_message = format_binance_json_command("SUBSCRIBE", symbol, index);
        }

        let shared_writer = Arc::clone(&shared_socket);
        let thread_handle = thread::spawn(move || -> () {
            shared_writer
                .lock()
                .unwrap()
                .send(Message::Text(subscribe_message))
                .unwrap();
        });
        thread_handle_vec.push(thread_handle);
    }

    thread::park();
}

fn create_ws_url(token: String, ws_end_point: String) -> String {
    return format!("{}?token={}&connectId=0001", ws_end_point, token)
}

fn dump_headers(response: &HttpResponse<Option<Vec<u8>>>) {
    println!("Connected to the binance server - OK!");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }
}
