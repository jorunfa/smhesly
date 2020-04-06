extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Smhesly")
        .version("0.1")
        .about("Send SMS using WGTWO API")
        .arg(
            Arg::with_name("from")
                .short("f")
                .long("from")
                .value_name("FROM")
                .help("Sender of SMS")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .value_name("TO")
                .help("Receiver of SMS")
                .takes_value(true)
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("MESSAGE")
                .help("SMS body content")
                .takes_value(true)
                .required(true)
                .index(3),
        )
        .get_matches();

    let from = matches.value_of("from").unwrap().to_owned();
    let to = matches.value_of("to").unwrap().to_owned();
    let content = matches.value_of("message").unwrap().to_owned();

    smhesly::send_sms(from, to, content).expect("Failed to send SMS");
}
