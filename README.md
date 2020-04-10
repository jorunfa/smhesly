# Smhesly

CLI written in Rust to send SMS using WGTWO APIs.

## Usage

```bash
$ smhesly 4799900111 4799900112 Hello world 👋
```

```
Smhesly 0.1.0
Send SMS using WGTWO API

USAGE:
    smhesly [FLAGS] <FROM> <TO> <MESSAGE>...

FLAGS:
    -d, --dryrun     Don't send SMS, only prints input
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FROM>          Sender of SMS (e.g.: 4799900111 or MyTelco)
    <TO>            Receiver of SMS (e.g.: 4799900111)
    <MESSAGE>...    SMS body content

```

## Install
```bash
$ cargo run -- --help # alt 1
$ cargo build && target/debug/smhesly --help # alt 2
$ cargo build --release && target/release/smhesly --help # alt 3
$ cargo install --path . && smhesly --help # alt 4
```

## Requirements

`BASE64_CLIENT_AND_SECRET` env variable needs to be set to use the CLI. This is the Client ID and secret, concatenated with a colon (:), and then base 64 encoded.

## Manual dependencies

* The `messagecore.proto` defenition is manually fetched from https://github.com/working-group-two/wgtwoapis/raw/master/wgtwo/messaging/messagecore.proto
* The `roots.pem` file needed for TLS is manually fetched from https://github.com/grpc/grpc/raw/master/etc/roots.pem
