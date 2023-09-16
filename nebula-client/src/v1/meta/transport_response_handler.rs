use std::io::{Cursor, Error as IoError, ErrorKind as IoErrorKind};

use fbthrift::{
    binary_protocol::BinaryProtocolDeserializer, ApplicationException, Deserialize, MessageType,
    ProtocolReader,
};
use fbthrift_transport_response_handler::ResponseHandler;
use nebula_fbthrift_meta_v1::services::meta_service::{
    GetSpaceExn, ListEdgesExn, ListPartsExn, ListSpacesExn, ListTagsExn,
};

#[derive(Clone)]
pub struct MetaTransportResponseHandler;

impl ResponseHandler for MetaTransportResponseHandler {
    fn try_make_static_response_bytes(
        &mut self,
        _service_name: &'static [u8],
        fn_name: &'static [u8],
        _request_bytes: &[u8],
    ) -> Result<Option<Vec<u8>>, IoError> {
        match fn_name {
            b"MetaService.listSpaces"
            | b"MetaService.getSpace"
            | b"MetaService.listParts"
            | b"MetaService.listTags"
            | b"MetaService.listEdges" => Ok(None),
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
            b"listSpaces" | b"getSpace" | b"listParts" | b"listTags" | b"listEdges" => {}
            _ => return Ok(None),
        };

        match message_type {
            MessageType::Reply => {
                match &name[..] {
                    b"listSpaces" => {
                        let _: ListSpacesExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"getSpace" => {
                        let _: GetSpaceExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"listParts" => {
                        let _: ListPartsExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"listTags" => {
                        let _: ListTagsExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"listEdges" => {
                        let _: ListEdgesExn = match Deserialize::read(&mut des) {
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
        let mut handler = MetaTransportResponseHandler;

        assert_eq!(
            handler.try_make_static_response_bytes(
                b"MetaService",
                b"MetaService.listSpaces",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                b"MetaService",
                b"MetaService.getSpace",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                b"MetaService",
                b"MetaService.listParts",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                b"MetaService",
                b"MetaService.listTags",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                b"MetaService",
                b"MetaService.listEdges",
                b"FOO"
            )?,
            None
        );
        match handler.try_make_static_response_bytes(b"MetaService", b"MetaService.foo", b"FOO") {
            Ok(_) => panic!(),
            Err(err) => {
                assert_eq!(err.kind(), IoErrorKind::Other);

                assert_eq!(err.to_string(), "Unknown method MetaService.foo");
            }
        }

        Ok(())
    }
}
