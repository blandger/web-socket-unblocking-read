
pub fn format_json_command(command: &str, symbol: &str, id: usize) -> String {
    let left_quote = "{";
    let right_quote = "}";
    // let body = format!(r#"{left_quote}"{key}":"{value}"{right_quote}"#);
    let formatted_command = match command {
        "SUBSCRIBE" => {
            // format!(r#"{left_quote}"method":"{command}","params":[{symbol}@aggTrade", "{symbol}@depth"], "id": {id}{right_quote}"#)
            format!(r#"{left_quote}"method":"{command}","params":["{symbol}@aggTrade"], "id": {id}{right_quote}"#)
        },
        "UNSUBSCRIBE" => {
            format!(r#"{left_quote}"method":"{command}","params":["{symbol}@depth"], "id": {id}{right_quote}"#)
        },
        _ => {
            panic!("unknown command !!")
        },
    };
    // let body = format!(SUBSCRIBE_TRADE_RQ, key, value);
    formatted_command
}
