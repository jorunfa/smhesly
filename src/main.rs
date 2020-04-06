use std::env;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder, CallOption, MetadataBuilder, ChannelCredentialsBuilder};

use protos::messagecore_grpc::MessageCoreClient;
use protos::messagecore::{TextMessage, Address, Address_Type, Direction};

fn main() {
    let mut from = Address::new();
    from.set_field_type(Address_Type::INTERNATIONAL_NUMBER);
    from.set_number(env::var("FROM").unwrap());

    let mut to = Address::new();
    to.set_field_type(Address_Type::INTERNATIONAL_NUMBER);
    to.set_number(env::var("TO").unwrap());

    let mut tm = TextMessage::new();
    tm.set_fromAddress(from);
    tm.set_toAddress(to);
    tm.set_body(env::var("CONTENT").unwrap());
    tm.set_direction(Direction::OUTGOING);

    let env = Arc::new(EnvBuilder::new().build());

    // This is needed because otherwise it says it fails to load /usr/share/grpc/roots.pem, which doesn't exist (on my Mac at least)
    let cert: Vec<u8> = include_bytes!("etc/roots.pem").to_vec();

    let channel = ChannelBuilder::new(env).secure_connect("api.wgtwo.com", ChannelCredentialsBuilder::new().root_cert(cert).build());
    let client = MessageCoreClient::new(channel);

    let auth = env::var("BASE64_CLIENT_AND_SECRET").expect("Credentials missing");

    let mut md = MetadataBuilder::new();
    md.add_str("authorization", format!("Basic {}", auth).as_str()).expect("Failed to add auth to metadata");
    let md = md.build();

    let co = CallOption::default();
    let co = co.headers(md);

    client.send_text_message_opt(&tm, co).expect("Sending of TextMessage failed");
}
