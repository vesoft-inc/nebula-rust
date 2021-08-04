use crate::configuration::AsyncTransportConfiguration;

use super::Call;

//
use futures_lite::io::Cursor;
fn block_on<T>(future: impl std::future::Future<Output = T>) -> T {
    futures_lite::future::block_on(future)
}

#[cfg(test)]
#[path = "../tests/inner_tests/transport_call_future.rs"]
mod inner_tests;
