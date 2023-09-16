use std::io::{Cursor, Error as IoError, ErrorKind as IoErrorKind};

use bytes::BytesMut;
use fbthrift::{
    binary_protocol::{BinaryProtocolDeserializer, BinaryProtocolSerializer},
    ApplicationException, Deserialize, MessageType, ProtocolReader, ProtocolWriter, Serialize,
};
use fbthrift_transport_response_handler::ResponseHandler;
use nebula_fbthrift_graph_v2::services::graph_service::{
    AuthenticateExn, ExecuteExn, ExecuteJsonExn, SignoutExn,
};

#[derive(Clone)]
pub struct GraphTransportResponseHandler;

impl ResponseHandler for GraphTransportResponseHandler {
    fn try_make_static_response_bytes(
        &mut self,
        _service_name: &'static [u8],
        fn_name: &'static [u8],
        request_bytes: &[u8],
    ) -> Result<Option<Vec<u8>>, IoError> {
        match fn_name {
            b"GraphService.authenticate" => Ok(None),
            b"GraphService.signout" => {
                let mut des = BinaryProtocolDeserializer::new(Cursor::new(request_bytes));
                let (name, message_type, seqid) = des
                    .read_message_begin(|v| v.to_vec())
                    .map_err(|err| IoError::new(IoErrorKind::Other, err))?;

                if name != b"signout" {
                    return Err(IoError::new(
                        IoErrorKind::Other,
                        format!("Unexpected name {name:?}"),
                    ));
                }

                if message_type != MessageType::Call {
                    return Err(IoError::new(
                        IoErrorKind::Other,
                        format!("Unexpected message type {message_type:?}"),
                    ));
                }

                let buf = BytesMut::with_capacity(1024);
                let mut ser = BinaryProtocolSerializer::<BytesMut>::with_buffer(buf);

                ser.write_message_begin("signout", MessageType::Reply, seqid);
                ser.write_message_end();

                SignoutExn::Success(()).write(&mut ser);

                let res_buf = ser.finish().to_vec();

                Ok(Some(res_buf))
            }
            b"GraphService.execute" => Ok(None),
            b"GraphService.executeJson" => Ok(None),
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
            b"authenticate" => {}
            b"signout" => unreachable!(),
            b"execute" => {}
            b"executeJson" => {}
            _ => return Ok(None),
        };

        match message_type {
            MessageType::Reply => {
                match &name[..] {
                    b"authenticate" => {
                        let _: AuthenticateExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"execute" => {
                        let _: ExecuteExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"executeJson" => {
                        let _: ExecuteJsonExn = match Deserialize::read(&mut des) {
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
        let mut handler = GraphTransportResponseHandler;

        assert_eq!(
            handler.try_make_static_response_bytes(
                b"GraphService",
                b"GraphService.authenticate",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                b"GraphService",
                b"GraphService.execute",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                b"GraphService",
                b"GraphService.executeJson",
                b"FOO"
            )?,
            None
        );
        match handler.try_make_static_response_bytes(b"GraphService", b"GraphService.foo", b"FOO") {
            Ok(_) => panic!(),
            Err(err) => {
                assert_eq!(err.kind(), IoErrorKind::Other);

                assert_eq!(err.to_string(), "Unknown method GraphService.foo");
            }
        }

        Ok(())
    }

    #[test]
    fn test_try_make_static_response_bytes_with_signout() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut handler = GraphTransportResponseHandler;

        //
        // Ref https://github.com/bk-rs/nebula-rs/blob/e500e6f93b0ffcd009038c2a51b41a6aa3488b18/nebula-fbthrift/nebula-fbthrift-graph-v2/src/lib.rs#L1346
        //
        let request = ::fbthrift::serialize!(::fbthrift::BinaryProtocol, |p| {
            p.write_message_begin("signout", ::fbthrift::MessageType::Call, 0);

            p.write_struct_begin("args");
            p.write_field_begin("arg_sessionId", ::fbthrift::TType::I64, 1i16);
            ::fbthrift::Serialize::write(&1, p);
            p.write_field_end();
            p.write_field_stop();
            p.write_struct_end();

            p.write_message_end();
        });

        match handler.try_make_static_response_bytes(
            b"GraphService",
            b"GraphService.signout",
            &request[..],
        ) {
            Ok(Some(_)) => {}
            _ => panic!(),
        }

        Ok(())
    }
}
