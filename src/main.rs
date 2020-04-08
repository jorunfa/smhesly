use clap::{App, Arg};

use smhesly::messagecore::{Address_Type, Direction};
use std::env;

fn main() {
    let matches = App::new("Smhesly")
        .version("0.1.0")
        .about("Send SMS using WGTWO API")
        .arg(
            Arg::with_name("from")
                .short("f")
                .long("from")
                .value_name("FROM")
                .help("Sender of SMS (e.g.: 4799900111 or MyTelco)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .value_name("TO")
                .help("Receiver of SMS (e.g.: 4799900111)")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("MESSAGE")
                .help("SMS body content")
                .required(true)
                .index(3),
        )
        .get_matches();

    let secret = env::var("BASE64_CLIENT_AND_SECRET").expect("Credentials missing");
    let from = matches.value_of("from").unwrap().to_owned();
    let to = matches.value_of("to").unwrap().to_owned();
    let content = matches.value_of("message").unwrap().to_owned();
    
    let from_type = match from.parse::<u32>() {
        Ok(_) => Address_Type::INTERNATIONAL_NUMBER,
        _ => Address_Type::TEXT,
    };

    let direction = match from_type {
        Address_Type::INTERNATIONAL_NUMBER => Direction::OUTGOING,
        _ => Direction::INCOMING,
    };

    smhesly::send_sms(secret, from, to, content, direction, from_type).expect("Failed to send SMS");
}
