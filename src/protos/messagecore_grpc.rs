// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_MESSAGE_CORE_SEND_MESSAGE: ::grpcio::Method<super::messagecore::Message, super::messagecore::SendResult> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/messaging.MessageCore/SendMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MESSAGE_CORE_SEND_TEXT_MESSAGE: ::grpcio::Method<super::messagecore::TextMessage, super::messagecore::SendResult> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/messaging.MessageCore/SendTextMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MESSAGE_CORE_RECEIVE_MESSAGES: ::grpcio::Method<super::messagecore::ReceiveMessagesRequest, super::messagecore::MessageBox> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/messaging.MessageCore/ReceiveMessages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MESSAGE_CORE_RECEIVE_TEXT_MESSAGES: ::grpcio::Method<super::messagecore::ReceiveTextMessagesRequest, super::messagecore::TextMessageBox> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/messaging.MessageCore/ReceiveTextMessages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MESSAGE_CORE_ACK_MESSAGE: ::grpcio::Method<super::messagecore::AckMessageRequest, super::messagecore::AckMessageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/messaging.MessageCore/AckMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MESSAGE_CORE_UNSUSPEND_ADDRESS: ::grpcio::Method<super::messagecore::Address, super::messagecore::UnsuspendResult> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/messaging.MessageCore/UnsuspendAddress",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MessageCoreClient {
    client: ::grpcio::Client,
}

impl MessageCoreClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MessageCoreClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_message_opt(&self, req: &super::messagecore::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::messagecore::SendResult> {
        self.client.unary_call(&METHOD_MESSAGE_CORE_SEND_MESSAGE, req, opt)
    }

    pub fn send_message(&self, req: &super::messagecore::Message) -> ::grpcio::Result<super::messagecore::SendResult> {
        self.send_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_message_async_opt(&self, req: &super::messagecore::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messagecore::SendResult>> {
        self.client.unary_call_async(&METHOD_MESSAGE_CORE_SEND_MESSAGE, req, opt)
    }

    pub fn send_message_async(&self, req: &super::messagecore::Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messagecore::SendResult>> {
        self.send_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_text_message_opt(&self, req: &super::messagecore::TextMessage, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::messagecore::SendResult> {
        self.client.unary_call(&METHOD_MESSAGE_CORE_SEND_TEXT_MESSAGE, req, opt)
    }

    pub fn send_text_message(&self, req: &super::messagecore::TextMessage) -> ::grpcio::Result<super::messagecore::SendResult> {
        self.send_text_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_text_message_async_opt(&self, req: &super::messagecore::TextMessage, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messagecore::SendResult>> {
        self.client.unary_call_async(&METHOD_MESSAGE_CORE_SEND_TEXT_MESSAGE, req, opt)
    }

    pub fn send_text_message_async(&self, req: &super::messagecore::TextMessage) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messagecore::SendResult>> {
        self.send_text_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn receive_messages_opt(&self, req: &super::messagecore::ReceiveMessagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::messagecore::MessageBox>> {
        self.client.server_streaming(&METHOD_MESSAGE_CORE_RECEIVE_MESSAGES, req, opt)
    }

    pub fn receive_messages(&self, req: &super::messagecore::ReceiveMessagesRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::messagecore::MessageBox>> {
        self.receive_messages_opt(req, ::grpcio::CallOption::default())
    }

    pub fn receive_text_messages_opt(&self, req: &super::messagecore::ReceiveTextMessagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::messagecore::TextMessageBox>> {
        self.client.server_streaming(&METHOD_MESSAGE_CORE_RECEIVE_TEXT_MESSAGES, req, opt)
    }

    pub fn receive_text_messages(&self, req: &super::messagecore::ReceiveTextMessagesRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::messagecore::TextMessageBox>> {
        self.receive_text_messages_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ack_message_opt(&self, req: &super::messagecore::AckMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::messagecore::AckMessageResponse> {
        self.client.unary_call(&METHOD_MESSAGE_CORE_ACK_MESSAGE, req, opt)
    }

    pub fn ack_message(&self, req: &super::messagecore::AckMessageRequest) -> ::grpcio::Result<super::messagecore::AckMessageResponse> {
        self.ack_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ack_message_async_opt(&self, req: &super::messagecore::AckMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messagecore::AckMessageResponse>> {
        self.client.unary_call_async(&METHOD_MESSAGE_CORE_ACK_MESSAGE, req, opt)
    }

    pub fn ack_message_async(&self, req: &super::messagecore::AckMessageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messagecore::AckMessageResponse>> {
        self.ack_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unsuspend_address_opt(&self, req: &super::messagecore::Address, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::messagecore::UnsuspendResult> {
        self.client.unary_call(&METHOD_MESSAGE_CORE_UNSUSPEND_ADDRESS, req, opt)
    }

    pub fn unsuspend_address(&self, req: &super::messagecore::Address) -> ::grpcio::Result<super::messagecore::UnsuspendResult> {
        self.unsuspend_address_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unsuspend_address_async_opt(&self, req: &super::messagecore::Address, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messagecore::UnsuspendResult>> {
        self.client.unary_call_async(&METHOD_MESSAGE_CORE_UNSUSPEND_ADDRESS, req, opt)
    }

    pub fn unsuspend_address_async(&self, req: &super::messagecore::Address) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messagecore::UnsuspendResult>> {
        self.unsuspend_address_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait MessageCore {
    fn send_message(&mut self, ctx: ::grpcio::RpcContext, req: super::messagecore::Message, sink: ::grpcio::UnarySink<super::messagecore::SendResult>);
    fn send_text_message(&mut self, ctx: ::grpcio::RpcContext, req: super::messagecore::TextMessage, sink: ::grpcio::UnarySink<super::messagecore::SendResult>);
    fn receive_messages(&mut self, ctx: ::grpcio::RpcContext, req: super::messagecore::ReceiveMessagesRequest, sink: ::grpcio::ServerStreamingSink<super::messagecore::MessageBox>);
    fn receive_text_messages(&mut self, ctx: ::grpcio::RpcContext, req: super::messagecore::ReceiveTextMessagesRequest, sink: ::grpcio::ServerStreamingSink<super::messagecore::TextMessageBox>);
    fn ack_message(&mut self, ctx: ::grpcio::RpcContext, req: super::messagecore::AckMessageRequest, sink: ::grpcio::UnarySink<super::messagecore::AckMessageResponse>);
    fn unsuspend_address(&mut self, ctx: ::grpcio::RpcContext, req: super::messagecore::Address, sink: ::grpcio::UnarySink<super::messagecore::UnsuspendResult>);
}

pub fn create_message_core<S: MessageCore + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MESSAGE_CORE_SEND_MESSAGE, move |ctx, req, resp| {
        instance.send_message(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MESSAGE_CORE_SEND_TEXT_MESSAGE, move |ctx, req, resp| {
        instance.send_text_message(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_MESSAGE_CORE_RECEIVE_MESSAGES, move |ctx, req, resp| {
        instance.receive_messages(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_MESSAGE_CORE_RECEIVE_TEXT_MESSAGES, move |ctx, req, resp| {
        instance.receive_text_messages(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MESSAGE_CORE_ACK_MESSAGE, move |ctx, req, resp| {
        instance.ack_message(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_MESSAGE_CORE_UNSUSPEND_ADDRESS, move |ctx, req, resp| {
        instance.unsuspend_address(ctx, req, resp)
    });
    builder.build()
}
