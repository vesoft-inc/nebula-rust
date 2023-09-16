use std::io::{Cursor, Error as IoError, ErrorKind as IoErrorKind};

use fbthrift::{
    binary_protocol::BinaryProtocolDeserializer, ApplicationException, Deserialize, MessageType,
    ProtocolReader,
};
use fbthrift_transport_response_handler::ResponseHandler;
use nebula_fbthrift_storage_v1::services::storage_service::{ScanEdgeExn, ScanVertexExn};

#[derive(Clone)]
pub struct StorageTransportResponseHandler;

impl ResponseHandler for StorageTransportResponseHandler {
    fn try_make_static_response_bytes(
        &mut self,
        _service_name: &'static [u8],
        fn_name: &'static [u8],
        _request_bytes: &[u8],
    ) -> Result<Option<Vec<u8>>, IoError> {
        match fn_name {
            b"StorageService.scanVertex" | b"StorageService.scanEdge" => Ok(None),
            _ => Err(IoError::new(
                IoErrorKind::Other,
                format!("Unknown method {}", String::from_utf8_lossy(fn_name)),
            )),
        }
    }

    fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> Result<Option<usize>, IoError> {
        let mut des = BinaryProtocolDeserializer::new(Cursor::new(response_bytes));
        let (name, message_type, _) = match des.read_message_begin(|v| v.to_vec()) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        match &name[..] {
            b"scanVertex" | b"scanEdge" => {}
            _ => return Ok(None),
        };

        match message_type {
            MessageType::Reply => {
                match &name[..] {
                    b"scanVertex" => {
                        let _: ScanVertexExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"scanEdge" => {
                        let _: ScanEdgeExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    _ => unreachable!(),
                };
            }
            MessageType::Exception => {
                let _: ApplicationException = match Deserialize::read(&mut des) {
                    Ok(v) => v,
                    Err(_) => return Ok(None),
                };
            }
            MessageType::Call | MessageType::Oneway | MessageType::InvalidMessageType => {}
        }

        match des.read_message_end() {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        Ok(Some(des.into_inner().position() as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_make_static_response_bytes() -> Result<(), Box<dyn std::error::Error>> {
        let mut handler = StorageTransportResponseHandler;

        assert_eq!(
            handler.try_make_static_response_bytes(
                b"StorageService",
                b"StorageService.scanVertex",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                b"StorageService",
                b"StorageService.scanEdge",
                b"FOO"
            )?,
            None
        );
        match handler.try_make_static_response_bytes(
            b"StorageService",
            b"StorageService.foo",
            b"FOO",
        ) {
            Ok(_) => panic!(),
            Err(err) => {
                assert_eq!(err.kind(), IoErrorKind::Other);

                assert_eq!(err.to_string(), "Unknown method StorageService.foo");
            }
        }

        Ok(())
    }
}
