use clap::{App, AppSettings, Arg};

use smhesly::messagecore::{Address_Type, Direction};
use std::env;

fn main() {
    let matches = App::new("Smhesly")
        .version("0.1.0")
        .about("Send SMS using WGTWO API")
        .arg(
            Arg::with_name("dryrun")
                .short("dr")
                .long("dryrun")
                .value_name("DRYRUN")
                .help("Don't send SMS, only prints input")
                .takes_value(false)
        )
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
                .multiple(true)
                .index(3),
        )
        .settings(&[AppSettings::TrailingVarArg])
        .get_matches();

    let secret = env::var("BASE64_CLIENT_AND_SECRET").expect("Credentials missing");
    let dry_run = matches.is_present("dryrun");
    let from = matches.value_of("from").unwrap().to_owned();
    let to = matches.value_of("to").unwrap().to_owned();
    let content: Vec<&str> = matches.values_of("message").unwrap().collect();
    let content = content.join(" ");

    let from_type = match from.parse::<usize>() {
        Ok(_) => Address_Type::INTERNATIONAL_NUMBER,
        _ => Address_Type::TEXT,
    };

    let direction = match from_type {
        Address_Type::INTERNATIONAL_NUMBER => Direction::OUTGOING,
        _ => Direction::INCOMING,
    };

    if dry_run {
        println!("From: {}", from);
        println!("From type: {:?}", from_type);
        println!("To: {}", to);
        println!("Direction {:?}", direction);
        println!("Message {:?}", content);
        return;
    }

    smhesly::send_sms(secret, from, to, content, direction, from_type).expect("Failed to send SMS");
}
