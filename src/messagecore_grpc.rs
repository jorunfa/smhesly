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


// interface

pub trait MessageCore {
    fn send_message(&self, o: ::grpc::RequestOptions, p: super::messagecore::Message) -> ::grpc::SingleResponse<super::messagecore::SendResult>;

    fn send_text_message(&self, o: ::grpc::RequestOptions, p: super::messagecore::TextMessage) -> ::grpc::SingleResponse<super::messagecore::SendResult>;

    fn receive_messages(&self, o: ::grpc::RequestOptions, p: super::messagecore::ReceiveMessagesRequest) -> ::grpc::StreamingResponse<super::messagecore::MessageBox>;

    fn receive_text_messages(&self, o: ::grpc::RequestOptions, p: super::messagecore::ReceiveTextMessagesRequest) -> ::grpc::StreamingResponse<super::messagecore::TextMessageBox>;

    fn ack_message(&self, o: ::grpc::RequestOptions, p: super::messagecore::AckMessageRequest) -> ::grpc::SingleResponse<super::messagecore::AckMessageResponse>;

    fn unsuspend_address(&self, o: ::grpc::RequestOptions, p: super::messagecore::Address) -> ::grpc::SingleResponse<super::messagecore::UnsuspendResult>;
}

// client

pub struct MessageCoreClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_SendMessage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::messagecore::Message, super::messagecore::SendResult>>,
    method_SendTextMessage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::messagecore::TextMessage, super::messagecore::SendResult>>,
    method_ReceiveMessages: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::messagecore::ReceiveMessagesRequest, super::messagecore::MessageBox>>,
    method_ReceiveTextMessages: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::messagecore::ReceiveTextMessagesRequest, super::messagecore::TextMessageBox>>,
    method_AckMessage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::messagecore::AckMessageRequest, super::messagecore::AckMessageResponse>>,
    method_UnsuspendAddress: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::messagecore::Address, super::messagecore::UnsuspendResult>>,
}

impl ::grpc::ClientStub for MessageCoreClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        MessageCoreClient {
            grpc_client: grpc_client,
            method_SendMessage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/messaging.MessageCore/SendMessage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendTextMessage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/messaging.MessageCore/SendTextMessage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ReceiveMessages: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/messaging.MessageCore/ReceiveMessages".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ReceiveTextMessages: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/messaging.MessageCore/ReceiveTextMessages".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AckMessage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/messaging.MessageCore/AckMessage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnsuspendAddress: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/messaging.MessageCore/UnsuspendAddress".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl MessageCore for MessageCoreClient {
    fn send_message(&self, o: ::grpc::RequestOptions, p: super::messagecore::Message) -> ::grpc::SingleResponse<super::messagecore::SendResult> {
        self.grpc_client.call_unary(o, p, self.method_SendMessage.clone())
    }

    fn send_text_message(&self, o: ::grpc::RequestOptions, p: super::messagecore::TextMessage) -> ::grpc::SingleResponse<super::messagecore::SendResult> {
        self.grpc_client.call_unary(o, p, self.method_SendTextMessage.clone())
    }

    fn receive_messages(&self, o: ::grpc::RequestOptions, p: super::messagecore::ReceiveMessagesRequest) -> ::grpc::StreamingResponse<super::messagecore::MessageBox> {
        self.grpc_client.call_server_streaming(o, p, self.method_ReceiveMessages.clone())
    }

    fn receive_text_messages(&self, o: ::grpc::RequestOptions, p: super::messagecore::ReceiveTextMessagesRequest) -> ::grpc::StreamingResponse<super::messagecore::TextMessageBox> {
        self.grpc_client.call_server_streaming(o, p, self.method_ReceiveTextMessages.clone())
    }

    fn ack_message(&self, o: ::grpc::RequestOptions, p: super::messagecore::AckMessageRequest) -> ::grpc::SingleResponse<super::messagecore::AckMessageResponse> {
        self.grpc_client.call_unary(o, p, self.method_AckMessage.clone())
    }

    fn unsuspend_address(&self, o: ::grpc::RequestOptions, p: super::messagecore::Address) -> ::grpc::SingleResponse<super::messagecore::UnsuspendResult> {
        self.grpc_client.call_unary(o, p, self.method_UnsuspendAddress.clone())
    }
}

// server

pub struct MessageCoreServer;


impl MessageCoreServer {
    pub fn new_service_def<H : MessageCore + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/messaging.MessageCore",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/messaging.MessageCore/SendMessage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_message(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/messaging.MessageCore/SendTextMessage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_text_message(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/messaging.MessageCore/ReceiveMessages".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.receive_messages(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/messaging.MessageCore/ReceiveTextMessages".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.receive_text_messages(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/messaging.MessageCore/AckMessage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.ack_message(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/messaging.MessageCore/UnsuspendAddress".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unsuspend_address(o, p))
                    },
                ),
            ],
        )
    }
}
