pub mod messagecore;
pub mod messagecore_grpc;

use std::sync::Arc;
use std::str::FromStr;

use grpcio::{ChannelBuilder, EnvBuilder, CallOption, MetadataBuilder, ChannelCredentialsBuilder};

use messagecore_grpc::MessageCoreClient;
use messagecore::{TextMessage, Address, Address_Type, Direction};

pub fn send_sms(auth: String, from: String, to: String, content: String, direction: Direction, from_type: Address_Type) -> Result<messagecore::SendResult, grpcio::Error> {
    let mut fa = Address::new();
    fa.set_field_type(from_type);
    fa.set_number(from);

    let mut ta = Address::new();
    ta.set_field_type(Address_Type::INTERNATIONAL_NUMBER);
    ta.set_number(to);

    let mut tm = TextMessage::new();
    tm.set_fromAddress(fa);
    tm.set_toAddress(ta);
    tm.set_body(content);

    tm.set_direction(direction);

    // This is needed because otherwise it says it fails to load /usr/share/grpc/roots.pem, which doesn't exist (on my Mac at least)
    let cert: Vec<u8> = include_bytes!("../etc/roots.pem").to_vec();

    let channel = ChannelBuilder::new(
        Arc::new(EnvBuilder::new().build())
    ).secure_connect("api.wgtwo.com", ChannelCredentialsBuilder::new().root_cert(cert).build());
    let client = MessageCoreClient::new(channel);


    let mut md = MetadataBuilder::new();
    md.add_str("authorization", format!("Basic {}", auth).as_str()).expect("Failed to add auth to metadata");
    let md = md.build();

    let co = CallOption::default();
    let co = co.headers(md);

    client.send_text_message_opt(&tm, co)
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "OUTGOING" => Ok(Direction::OUTGOING),
            "INCOMING" => Ok(Direction::INCOMING),
            _ => Err(()),
        }
    }
}
