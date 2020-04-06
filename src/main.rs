use std::env;

fn main() {
    let from = env::var("FROM").unwrap();
    let to = env::var("TO").unwrap();
    let content = env::var("CONTENT").unwrap();

    smhesly::send_sms(from, to, content).expect("Failed to send SMS");
}
