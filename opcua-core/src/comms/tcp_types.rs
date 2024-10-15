// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains the implementation of various UA over TCP types.

use std::io::{Cursor, Error, ErrorKind, Read, Result, Write};

use log::error;
use opcua_types::{
    encoding::*, service_types::EndpointDescription, status_code::StatusCode, string::UAString,
};

use super::url::url_matches_except_host;

pub const CHUNK_MESSAGE: &[u8] = b"MSG";
pub const OPEN_SECURE_CHANNEL_MESSAGE: &[u8] = b"OPN";
pub const CLOSE_SECURE_CHANNEL_MESSAGE: &[u8] = b"CLO";

const HELLO_MESSAGE: &[u8] = b"HEL";
const ACKNOWLEDGE_MESSAGE: &[u8] = b"ACK";
const ERROR_MESSAGE: &[u8] = b"ERR";

pub const CHUNK_FINAL: u8 = b'F';
pub const CHUNK_INTERMEDIATE: u8 = b'C';
pub const CHUNK_FINAL_ERROR: u8 = b'A';

/// Minimum size in bytes than any single message chunk can be
pub const MIN_CHUNK_SIZE: usize = 8192;

/// Size in bytes of an OPC UA message header
pub const MESSAGE_HEADER_LEN: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Invalid,
    Hello,
    Acknowledge,
    Chunk,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeader {
    pub message_type: MessageType,
    pub message_size: u32,
}

impl BinaryEncoder for MessageHeader {
    fn byte_len(&self) -> usize {
        MESSAGE_HEADER_LEN
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size: usize = 0;
        let result = match self.message_type {
            MessageType::Hello => stream.write(HELLO_MESSAGE),
            MessageType::Acknowledge => stream.write(ACKNOWLEDGE_MESSAGE),
            MessageType::Error => stream.write(ERROR_MESSAGE),
            MessageType::Chunk => {
                panic!("Don't write chunks to stream with this call, use Chunk and Chunker");
            }
            _ => {
                panic!("Unrecognized type");
            }
        };
        size += process_encode_io_result(result)?;
        size += write_u8(stream, b'F')?;
        size += write_u32(stream, self.message_size)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        let mut message_type = [0u8; 4];
        process_decode_io_result(stream.read_exact(&mut message_type))?;
        let message_size = read_u32(stream)?;
        Ok(MessageHeader {
            message_type: MessageHeader::message_type(&message_type),
            message_size,
        })
    }
}

impl MessageHeader {
    pub fn new(message_type: MessageType) -> MessageHeader {
        MessageHeader {
            message_type,
            message_size: 0,
        }
    }

    /// Reads the bytes of the stream to a buffer. If first 4 bytes are invalid,
    /// code returns an error
    pub fn read_bytes<S: Read>(
        stream: &mut S,
        decoding_options: &DecodingOptions,
    ) -> Result<Vec<u8>> {
        // Read the bytes of the stream into a vector
        let mut header = [0u8; 4];
        stream.read_exact(&mut header)?;
        if MessageHeader::message_type(&header) == MessageType::Invalid {
            return Err(Error::new(
                ErrorKind::Other,
                "Message type is not recognized, cannot read bytes",
            ));
        }
        let message_size = u32::decode(stream, decoding_options);
        if message_size.is_err() {
            return Err(Error::new(ErrorKind::Other, "Cannot decode message_size"));
        }
        let message_size = message_size.unwrap();

        // Write header to stream
        let mut out = Cursor::new(Vec::with_capacity(message_size as usize));
        let result = out.write(&header);
        if result.is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                "Cannot write message header to buffer ",
            ));
        }

        let result = message_size.encode(&mut out);
        if result.is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                "Cannot write message size to buffer ",
            ));
        }

        let pos = out.position() as usize;
        // Read remaining bytes straight into the vec
        let mut result = out.into_inner();
        result.resize(message_size as usize, 0u8);
        stream.read_exact(&mut result[pos..])?;

        Ok(result)
    }

    pub fn message_type(t: &[u8]) -> MessageType {
        if t.len() != 4 {
            MessageType::Invalid
        } else {
            let message_type = match &t[0..3] {
                HELLO_MESSAGE => MessageType::Hello,
                ACKNOWLEDGE_MESSAGE => MessageType::Acknowledge,
                ERROR_MESSAGE => MessageType::Error,
                CHUNK_MESSAGE | OPEN_SECURE_CHANNEL_MESSAGE | CLOSE_SECURE_CHANNEL_MESSAGE => {
                    MessageType::Chunk
                }
                _ => {
                    error!("message type doesn't match anything");
                    MessageType::Invalid
                }
            };

            // Check the 4th byte which should be F for messages or F, C or A for chunks. If its
            // not one of those, the message is invalid
            match t[3] {
                CHUNK_FINAL => message_type,
                CHUNK_INTERMEDIATE | CHUNK_FINAL_ERROR => {
                    if message_type == MessageType::Chunk {
                        message_type
                    } else {
                        MessageType::Invalid
                    }
                }
                _ => MessageType::Invalid,
            }
        }
    }
}

/// Implementation of the HEL message in OPC UA
#[derive(Debug, Clone, PartialEq)]
pub struct HelloMessage {
    message_header: MessageHeader,
    pub protocol_version: u32,
    pub receive_buffer_size: u32,
    pub send_buffer_size: u32,
    pub max_message_size: u32,
    pub max_chunk_count: u32,
    pub endpoint_url: UAString,
}

impl BinaryEncoder for HelloMessage {
    fn byte_len(&self) -> usize {
        // 5 * u32 = 20
        self.message_header.byte_len() + 20 + self.endpoint_url.byte_len()
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.message_header.encode(stream)?;
        size += self.protocol_version.encode(stream)?;
        size += self.receive_buffer_size.encode(stream)?;
        size += self.send_buffer_size.encode(stream)?;
        size += self.max_message_size.encode(stream)?;
        size += self.max_chunk_count.encode(stream)?;
        size += self.endpoint_url.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let message_header = MessageHeader::decode(stream, decoding_options)?;
        let protocol_version = u32::decode(stream, decoding_options)?;
        let receive_buffer_size = u32::decode(stream, decoding_options)?;
        let send_buffer_size = u32::decode(stream, decoding_options)?;
        let max_message_size = u32::decode(stream, decoding_options)?;
        let max_chunk_count = u32::decode(stream, decoding_options)?;
        let endpoint_url = UAString::decode(stream, decoding_options)?;
        Ok(HelloMessage {
            message_header,
            protocol_version,
            receive_buffer_size,
            send_buffer_size,
            max_message_size,
            max_chunk_count,
            endpoint_url,
        })
    }
}

impl HelloMessage {
    const MAX_URL_LEN: usize = 4096;

    /// Creates a HEL message
    pub fn new(
        endpoint_url: &str,
        send_buffer_size: usize,
        receive_buffer_size: usize,
        max_message_size: usize,
        max_chunk_count: usize,
    ) -> HelloMessage {
        let mut msg = HelloMessage {
            message_header: MessageHeader::new(MessageType::Hello),
            protocol_version: 0,
            send_buffer_size: send_buffer_size as u32,
            receive_buffer_size: receive_buffer_size as u32,
            max_message_size: max_message_size as u32,
            max_chunk_count: max_chunk_count as u32,
            endpoint_url: UAString::from(endpoint_url),
        };
        msg.message_header.message_size = msg.byte_len() as u32;
        msg
    }

    pub fn is_endpoint_url_valid(&self, endpoints: &[EndpointDescription]) -> bool {
        if self.is_endpoint_valid_length() {
            self.matches_endpoint(endpoints)
        } else {
            // Length > 4096
            error!("Supplied endpoint url exceeds maximum length");
            false
        }
    }

    pub fn is_endpoint_valid_length(&self) -> bool {
        if let Some(ref endpoint_url) = self.endpoint_url.value() {
            endpoint_url.len() <= HelloMessage::MAX_URL_LEN
        } else {
            error!("Hello message contains no endpoint url");
            false
        }
    }

    pub fn matches_endpoint(&self, endpoints: &[EndpointDescription]) -> bool {
        // check server's endpoints to find one that matches the hello
        endpoints.iter().any(|e| {
            // Server might have different hostname than that supplied by client, so
            // ignore that bit.
            url_matches_except_host(e.endpoint_url.as_ref(), self.endpoint_url.as_ref())
        })
    }

    pub fn is_valid_buffer_sizes(&self) -> bool {
        // Set in part 6 as minimum transport buffer size
        self.receive_buffer_size >= MIN_CHUNK_SIZE as u32
            && self.send_buffer_size >= MIN_CHUNK_SIZE as u32
    }
}

/// Implementation of the ACK message in OPC UA
#[derive(Debug, Clone, PartialEq)]
pub struct AcknowledgeMessage {
    message_header: MessageHeader,
    pub protocol_version: u32,
    pub receive_buffer_size: u32,
    pub send_buffer_size: u32,
    pub max_message_size: u32,
    pub max_chunk_count: u32,
}

impl BinaryEncoder for AcknowledgeMessage {
    fn byte_len(&self) -> usize {
        self.message_header.byte_len() + 20
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size: usize = 0;
        size += self.message_header.encode(stream)?;
        size += self.protocol_version.encode(stream)?;
        size += self.receive_buffer_size.encode(stream)?;
        size += self.send_buffer_size.encode(stream)?;
        size += self.max_message_size.encode(stream)?;
        size += self.max_chunk_count.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let message_header = MessageHeader::decode(stream, decoding_options)?;
        let protocol_version = u32::decode(stream, decoding_options)?;
        let receive_buffer_size = u32::decode(stream, decoding_options)?;
        let send_buffer_size = u32::decode(stream, decoding_options)?;
        let max_message_size = u32::decode(stream, decoding_options)?;
        let max_chunk_count = u32::decode(stream, decoding_options)?;
        Ok(AcknowledgeMessage {
            message_header,
            protocol_version,
            receive_buffer_size,
            send_buffer_size,
            max_message_size,
            max_chunk_count,
        })
    }
}

impl AcknowledgeMessage {
    pub fn new(
        protocol_version: u32,
        receive_buffer_size: u32,
        send_buffer_size: u32,
        max_message_size: u32,
        max_chunk_count: u32,
    ) -> Self {
        let mut ack = AcknowledgeMessage {
            message_header: MessageHeader::new(MessageType::Acknowledge),
            protocol_version,
            receive_buffer_size,
            send_buffer_size,
            max_message_size,
            max_chunk_count,
        };
        ack.message_header.message_size = ack.byte_len() as u32;
        ack
    }
}

/// Implementation of the ERR message in OPC UA
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorMessage {
    message_header: MessageHeader,
    pub error: u32,
    pub reason: UAString,
}

impl BinaryEncoder for ErrorMessage {
    fn byte_len(&self) -> usize {
        self.message_header.byte_len() + self.error.byte_len() + self.reason.byte_len()
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size: usize = 0;
        size += self.message_header.encode(stream)?;
        size += self.error.encode(stream)?;
        size += self.reason.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let message_header = MessageHeader::decode(stream, decoding_options)?;
        let error = u32::decode(stream, decoding_options)?;
        let reason = UAString::decode(stream, decoding_options)?;
        Ok(ErrorMessage {
            message_header,
            error,
            reason,
        })
    }
}

impl ErrorMessage {
    pub fn from_status_code(status_code: StatusCode) -> ErrorMessage {
        Self::new(status_code, status_code.sub_code().description())
    }

    pub fn new(status_code: StatusCode, reason: &str) -> ErrorMessage {
        let mut error = ErrorMessage {
            message_header: MessageHeader::new(MessageType::Error),
            error: status_code.bits(),
            reason: UAString::from(reason),
        };
        error.message_header.message_size = error.byte_len() as u32;
        error
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::comms::tcp_types::{
        AcknowledgeMessage, BinaryEncoder, HelloMessage, MessageHeader, MessageType,
    };
    use opcua_types::{
        ApplicationDescription, ByteString, DecodingOptions, EndpointDescription,
        MessageSecurityMode, UAString,
    };

    fn hello_data() -> Vec<u8> {
        vec![
            0x48, 0x45, 0x4c, 0x46, 0x39, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x0a, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x19, 0x00, 0x00, 0x00, 0x6f, 0x70, 0x63, 0x2e, 0x74, 0x63, 0x70, 0x3a, 0x2f, 0x2f,
            0x31, 0x32, 0x37, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x31, 0x3a, 0x31, 0x32, 0x33, 0x34,
            0x2f,
        ]
    }

    fn ack_data() -> Vec<u8> {
        vec![
            0x41, 0x43, 0x4b, 0x46, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x01, 0xff, 0xff, 0x00, 0x00,
        ]
    }

    #[test]
    pub fn hello() {
        let mut stream = Cursor::new(hello_data());
        let decoding_options = DecodingOptions::test();
        let hello = HelloMessage::decode(&mut stream, &decoding_options).unwrap();
        println!("hello = {:?}", hello);
        assert_eq!(hello.message_header.message_type, MessageType::Hello);
        assert_eq!(hello.message_header.message_size, 57);
        assert_eq!(hello.protocol_version, 0);
        assert_eq!(hello.receive_buffer_size, 655360);
        assert_eq!(hello.send_buffer_size, 655360);
        assert_eq!(hello.max_message_size, 0);
        assert_eq!(hello.max_chunk_count, 0);
        assert_eq!(
            hello.endpoint_url,
            UAString::from("opc.tcp://127.0.0.1:1234/")
        );
    }

    #[test]
    pub fn acknowledge() {
        let mut stream = Cursor::new(ack_data());
        let decoding_options = DecodingOptions::test();
        let ack = AcknowledgeMessage::decode(&mut stream, &decoding_options).unwrap();
        println!("ack = {:?}", ack);
        assert_eq!(ack.message_header.message_type, MessageType::Acknowledge);
        assert_eq!(ack.message_header.message_size, 28);
        assert_eq!(ack.protocol_version, 0);
        assert_eq!(ack.receive_buffer_size, 524288);
        assert_eq!(ack.send_buffer_size, 524288);
        assert_eq!(ack.max_message_size, 16777216);
        assert_eq!(ack.max_chunk_count, 65535);
    }

    #[test]
    fn endpoint_url() {
        // Ensure hello with None endpoint is invalid
        // Ensure hello with URL > 4096 chars is invalid
        let mut h = HelloMessage {
            message_header: MessageHeader {
                message_type: MessageType::Invalid,
                message_size: 0,
            },
            protocol_version: 0,
            receive_buffer_size: 0,
            send_buffer_size: 0,
            max_message_size: 0,
            max_chunk_count: 0,
            endpoint_url: UAString::null(),
        };

        let endpoints = vec![EndpointDescription {
            endpoint_url: UAString::from("opc.tcp://foo"),
            security_policy_uri: UAString::null(),
            security_mode: MessageSecurityMode::None,
            server: ApplicationDescription::default(),
            security_level: 0,
            server_certificate: ByteString::null(),
            transport_profile_uri: UAString::null(),
            user_identity_tokens: None,
        }];

        // Negative tests
        assert!(!h.matches_endpoint(&endpoints));
        h.endpoint_url = UAString::from("");
        assert!(!h.matches_endpoint(&endpoints));
        h.endpoint_url = UAString::from("opc.tcp://foo/blah");
        assert!(!h.matches_endpoint(&endpoints));
        // 4097 bytes
        h.endpoint_url = UAString::from((0..4097).map(|_| 'A').collect::<String>());
        assert!(!h.is_endpoint_valid_length());

        // Positive tests
        h.endpoint_url = UAString::from("opc.tcp://foo/");
        assert!(h.matches_endpoint(&endpoints));
        h.endpoint_url = UAString::from("opc.tcp://bar/"); // Ignore hostname
        assert!(h.matches_endpoint(&endpoints));
        h.endpoint_url = UAString::from((0..4096).map(|_| 'A').collect::<String>());
        assert!(h.is_endpoint_valid_length())
    }

    #[test]
    fn valid_buffer_sizes() {
        // Test that invalid buffer sizes are rejected, while valid buffer sizes are accepted
        let mut h = HelloMessage {
            message_header: MessageHeader {
                message_type: MessageType::Invalid,
                message_size: 0,
            },
            protocol_version: 0,
            receive_buffer_size: 0,
            send_buffer_size: 0,
            max_message_size: 0,
            max_chunk_count: 0,
            endpoint_url: UAString::null(),
        };
        assert!(!h.is_valid_buffer_sizes());
        h.receive_buffer_size = 8191;
        assert!(!h.is_valid_buffer_sizes());
        h.send_buffer_size = 8191;
        assert!(!h.is_valid_buffer_sizes());
        h.receive_buffer_size = 8192;
        assert!(!h.is_valid_buffer_sizes());
        h.send_buffer_size = 8192;
        assert!(h.is_valid_buffer_sizes());
    }
}