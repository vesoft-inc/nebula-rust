use std::io;

pub trait ResponseHandler: Clone {
    fn try_make_static_response_bytes(
        &mut self,
        request_bytes: &[u8],
    ) -> io::Result<Option<Vec<u8>>>;

    fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>>;
}

#[derive(Clone)]
pub struct DefaultResponseHandler;

impl ResponseHandler for DefaultResponseHandler {
    fn try_make_static_response_bytes(
        &mut self,
        _request_bytes: &[u8],
    ) -> io::Result<Option<Vec<u8>>> {
        Ok(None)
    }

    fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>> {
        Ok(Some(response_bytes.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io;

    #[test]
    fn with_default_response_handler() -> io::Result<()> {
        let mut h = DefaultResponseHandler;

        assert_eq!(h.try_make_static_response_bytes(&b""[..])?, None);

        assert_eq!(h.parse_response_bytes(&b"foo"[..])?, Some(3));

        Ok(())
    }
}
