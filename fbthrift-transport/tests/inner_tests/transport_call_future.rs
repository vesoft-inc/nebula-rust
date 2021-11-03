use super::*;

use std::io;
use std::panic;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use fbthrift_transport_response_handler::ResponseHandler;

#[test]
fn call_with_static_res() -> io::Result<()> {
    #[derive(Clone)]
    pub struct FooResponseHandler;

    impl ResponseHandler for FooResponseHandler {
        fn try_make_static_response_bytes(
            &mut self,
            request_bytes: &[u8],
        ) -> io::Result<Option<Vec<u8>>> {
            Ok(if request_bytes == b"static" {
                Some(b"bar".to_vec())
            } else {
                None
            })
        }

        fn parse_response_bytes(&mut self, _response_bytes: &[u8]) -> io::Result<Option<usize>> {
            unimplemented!()
        }
    }

    block_on(async {
        let mut buf = b"1234567890".to_vec();
        let cursor = Cursor::new(&mut buf);
        let stream = Arc::new(Mutex::new(cursor));
        let c = AsyncTransportConfiguration::new(FooResponseHandler);

        //
        let req = Bytes::from("static");
        let call = Call::new(stream.clone(), req, c.clone());

        let out = call.await.expect("");
        assert_eq!(out.into_inner(), Bytes::from("bar"));

        assert_eq!(stream.lock().expect("").get_ref(), &b"static7890");

        Ok(())
    })
}

#[test]
fn call_with_dynamic_res() -> io::Result<()> {
    #[derive(Clone)]
    pub struct FooResponseHandler;

    impl ResponseHandler for FooResponseHandler {
        fn try_make_static_response_bytes(
            &mut self,
            request_bytes: &[u8],
        ) -> io::Result<Option<Vec<u8>>> {
            Ok(if request_bytes == b"dynamic" {
                None
            } else {
                unimplemented!()
            })
        }

        fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>> {
            if response_bytes == b"89012" {
                Ok(Some(2))
            } else {
                unimplemented!()
            }
        }
    }

    block_on(async {
        let mut buf = b"123456789012".to_vec();
        let cursor = Cursor::new(&mut buf);
        let stream = Arc::new(Mutex::new(cursor));
        let c = AsyncTransportConfiguration::new(FooResponseHandler);

        //
        let req = Bytes::from("dynamic");
        let call = Call::new(stream.clone(), req, c.clone());

        let out = call.await.expect("");
        assert_eq!(out.into_inner(), Bytes::from("89"));

        assert_eq!(stream.lock().expect("").get_ref(), &b"dynamic89012");

        Ok(())
    })
}

#[test]
fn call_with_dynamic_res_and_less_buf_size() -> io::Result<()> {
    #[derive(Clone)]
    pub struct FooResponseHandler;

    impl ResponseHandler for FooResponseHandler {
        fn try_make_static_response_bytes(
            &mut self,
            request_bytes: &[u8],
        ) -> io::Result<Option<Vec<u8>>> {
            Ok(if request_bytes == b"dynamic" {
                None
            } else {
                unimplemented!()
            })
        }

        fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>> {
            if response_bytes == b"8" {
                Ok(None)
            } else if response_bytes == b"89" {
                Ok(None)
            } else if response_bytes == b"890" {
                Ok(None)
            } else if response_bytes == b"8901" {
                Ok(None)
            } else if response_bytes == b"89012" {
                Ok(Some(4))
            } else {
                unimplemented!()
            }
        }
    }

    block_on(async {
        let mut buf = b"123456789012".to_vec();
        let cursor = Cursor::new(&mut buf);
        let stream = Arc::new(Mutex::new(cursor));
        let mut c = AsyncTransportConfiguration::new(FooResponseHandler);
        c.set_buf_size(1);
        c.set_max_parse_response_bytes_count(99);

        //
        let req = Bytes::from("dynamic");
        let call = Call::new(stream.clone(), req, c.clone());

        let out = call.await.expect("");
        assert_eq!(out.into_inner(), Bytes::from("8901"));

        assert_eq!(stream.lock().expect("").get_ref(), &b"dynamic89012");

        Ok(())
    })
}

#[test]
fn call_with_dynamic_res_and_less_max_buf_size() -> io::Result<()> {
    #[derive(Clone)]
    pub struct FooResponseHandler;

    impl ResponseHandler for FooResponseHandler {
        fn try_make_static_response_bytes(
            &mut self,
            request_bytes: &[u8],
        ) -> io::Result<Option<Vec<u8>>> {
            Ok(if request_bytes == b"dynamic" {
                None
            } else {
                unimplemented!()
            })
        }

        fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>> {
            if response_bytes == b"8" {
                Ok(None)
            } else if response_bytes == b"89" {
                Ok(None)
            } else if response_bytes == b"890" {
                Ok(None)
            } else if response_bytes == b"8901" {
                Ok(None)
            } else if response_bytes == b"89012" {
                Ok(Some(4))
            } else {
                unimplemented!()
            }
        }
    }

    block_on(async {
        let mut buf = b"123456789012".to_vec();
        let cursor = Cursor::new(&mut buf);
        let stream = Arc::new(Mutex::new(cursor));
        let mut c = AsyncTransportConfiguration::new(FooResponseHandler);
        c.set_buf_size(1);
        c.set_max_buf_size(3);

        //
        let req = Bytes::from("dynamic");
        let call = Call::new(stream.clone(), req, c.clone());

        match call.await {
            Ok(_) => assert!(false),
            Err(err) => {
                assert!(err.to_string() == "Reach max buffer size");
            }
        }

        assert_eq!(stream.lock().expect("").get_ref(), &b"dynamic89012");

        Ok(())
    })
}

#[test]
fn call_with_dynamic_res_and_too_many_read() -> io::Result<()> {
    #[derive(Clone)]
    pub struct FooResponseHandler;

    impl ResponseHandler for FooResponseHandler {
        fn try_make_static_response_bytes(
            &mut self,
            request_bytes: &[u8],
        ) -> io::Result<Option<Vec<u8>>> {
            Ok(if request_bytes == b"dynamic" {
                None
            } else {
                unimplemented!()
            })
        }

        fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>> {
            if response_bytes == b"" {
                Ok(None)
            } else {
                unimplemented!()
            }
        }
    }

    block_on(async {
        let mut buf = b"".to_vec();
        let cursor = Cursor::new(&mut buf);
        let stream = Arc::new(Mutex::new(cursor));
        let c = AsyncTransportConfiguration::new(FooResponseHandler);

        //
        let req = Bytes::from("dynamic");
        let call = Call::new(stream.clone(), req, c.clone());

        match call.await {
            Ok(_) => assert!(false),
            Err(err) => {
                assert!(err.to_string() == "Reach max parse response bytes count");
            }
        }

        assert_eq!(stream.lock().expect("").get_ref(), &b"dynamic");

        Ok(())
    })
}
