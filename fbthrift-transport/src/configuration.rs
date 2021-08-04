use std::time::Duration;

use fbthrift_transport_response_handler::{DefaultResponseHandler, ResponseHandler};

#[derive(Clone)]
pub struct AsyncTransportConfiguration<H>
where
    H: ResponseHandler,
{
    buf_size: usize,
    max_buf_size: usize,
    read_timeout: Duration,
    max_parse_response_bytes_count: u8,
    pub(crate) response_handler: H,
}

impl<H> AsyncTransportConfiguration<H>
where
    H: ResponseHandler,
{
    pub fn new(response_handler: H) -> Self {
        Self {
            buf_size: 1024,
            max_buf_size: 1024 * 4,
            read_timeout: Duration::from_secs(5),
            max_parse_response_bytes_count: 3,
            response_handler,
        }
    }

    pub fn set_buf_size(&mut self, size: usize) {
        debug_assert!(size <= self.max_buf_size);
        self.buf_size = size;
    }

    pub fn get_buf_size(&self) -> usize {
        self.buf_size
    }

    pub fn set_max_buf_size(&mut self, size: usize) {
        debug_assert!(size >= self.buf_size);
        self.max_buf_size = size;
    }

    pub fn get_max_buf_size(&self) -> usize {
        self.max_buf_size
    }

    pub fn set_read_timeout(&mut self, timeout_ms: u32) {
        debug_assert!(timeout_ms > 0);
        self.read_timeout = Duration::from_millis(timeout_ms as u64);
    }

    pub fn get_read_timeout(&self) -> Duration {
        self.read_timeout
    }

    pub fn set_max_parse_response_bytes_count(&mut self, size: u8) {
        debug_assert!(size > 0);
        self.max_parse_response_bytes_count = size;
    }

    pub fn get_max_parse_response_bytes_count(&self) -> u8 {
        self.max_parse_response_bytes_count
    }
}

pub type DefaultAsyncTransportConfiguration = AsyncTransportConfiguration<DefaultResponseHandler>;

impl Default for DefaultAsyncTransportConfiguration {
    fn default() -> Self {
        Self::new(DefaultResponseHandler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io;

    #[test]
    fn get_and_set() -> io::Result<()> {
        let mut c: DefaultAsyncTransportConfiguration = Default::default();

        assert_eq!(c.get_buf_size(), 1024);
        assert_eq!(c.get_max_buf_size(), 1024 * 4);
        assert_eq!(c.get_read_timeout(), Duration::from_secs(5));
        assert_eq!(c.get_max_parse_response_bytes_count(), 3);

        c.set_buf_size(1024 * 2);
        assert_eq!(c.get_buf_size(), 1024 * 2);
        c.set_max_buf_size(1024 * 3);
        assert_eq!(c.get_max_buf_size(), 1024 * 3);
        c.set_read_timeout(3000);
        assert_eq!(c.get_read_timeout(), Duration::from_secs(3));
        c.set_max_parse_response_bytes_count(2);
        assert_eq!(c.get_max_parse_response_bytes_count(), 2);

        Ok(())
    }
}
