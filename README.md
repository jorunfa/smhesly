# Smhesly

CLI written in Rust to send SMS using WGTWO APIs.

## Usage

```bash
$ ./smhesly --help
Smhesly 0.1.0
Send SMS using WGTWO API

USAGE:
    smhesly <FROM> <TO> <MESSAGE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FROM>       Sender of SMS (e.g.: 4799900111 or MyTelco)
    <TO>         Receiver of SMS (e.g.: 4799900111)
    <MESSAGE>    SMS body content
```

## Requirements

`BASE64_CLIENT_AND_SECRET` env variable needs to be set to use the CLI. This is the Client ID and secret, concatenated with a colon (:), and then base 64 encoded.

## Manual dependencies

* The `messagecore.proto` defenition is manually fetched from https://github.com/working-group-two/wgtwoapis/raw/master/wgtwo/messaging/messagecore.proto
* The `roots.pem` file needed for TLS is manually fetched from https://github.com/grpc/grpc/raw/master/etc/roots.pem
