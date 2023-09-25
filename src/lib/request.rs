
pub fn format_binance_json_command(command: &str, symbol: &str, id: usize) -> String {
    let left_quote = "{";
    let right_quote = "}";
    // let body = format!(r#"{left_quote}"{key}":"{value}"{right_quote}"#);
    let symbol = symbol.replace("-", "");
    let formatted_command = match command {
        "SUBSCRIBE" => {
            // format!(r#"{left_quote}"method":"{command}","params":[{symbol}@aggTrade", "{symbol}@depth"], "id": {id}{right_quote}"#)
            format!(r#"{left_quote}"method":"{command}","params":["{symbol}@aggTrade"],"id":{id}{right_quote}"#)
        },
        "UNSUBSCRIBE" => {
            format!(r#"{left_quote}"method":"{command}","params":["{symbol}@depth"],"id":{id}{right_quote}"#)
        },
        _ => {
            panic!("unknown command !!")
        },
    };
    println!("binance command = {formatted_command}");
    formatted_command
}

pub fn format_kucoin_json_command(command: &str, symbol: &str, id: usize) -> String {
    let left_quote = "{";
    let right_quote = "}";
    let symbol = symbol.to_uppercase();
    let formatted_command = match command {
        "SUBSCRIBE" => {
        /*
        "id": 1545910660739,                          //The id should be an unique value
        "type": "subscribe",
        "topic": "/market/ticker:BTC-USDT,ETH-USDT",  //Topic needs to be subscribed. Some topics support to divisional subscribe the informations of multiple trading pairs through ",".
        "privateChannel": false,                      //Adopted the private channel or not. Set as false by default.
        "response": true
        */
            format!(r#"{left_quote}"type":"{command}","topic":"/market/ticker:{symbol}","id":"{id}","privateChannel":false,"response":true{right_quote}"#)
        },
        "UNSUBSCRIBE" => {
        /*
        {
            "id": "1545910840805",                            //The id should be an unique value
            "type": "unsubscribe",
            "topic": "/market/ticker:BTC-USDT,ETH-USDT",      //Topic needs to be unsubscribed. Some topics support to divisional unsubscribe the informations of multiple trading pairs through ",".
            "privateChannel": false,
            "response": true                                  //Whether the server needs to return the receipt information of this subscription or not. Set as false by default.
        }
        */
            format!(r#"{left_quote}"type":"{command}","topic":"/market/ticker:{symbol}","id":"{id}","privateChannel": false,"response": false{right_quote}"#)
        },
        _ => {
            panic!("unknown command !!")
        },
    };
    println!("kucoin command = {formatted_command}");
    formatted_command
}
