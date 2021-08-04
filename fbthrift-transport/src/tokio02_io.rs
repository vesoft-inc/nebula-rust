use futures_x_io::{
    tokio02_io::{AsyncRead, AsyncWrite},
    tokio02_io_util::AsyncWriteExt,
};
use futures_x_io_timeoutable::tokio02_io::rw::AsyncReadWithTimeoutExt;

//
use std::ops::Deref;
use std::pin::Pin;

fn pin_write_future<P>(write_future: P) -> Pin<P>
where
    P: Deref,
    <P as Deref>::Target: Unpin,
{
    Pin::new(write_future)
}

//
#[path = "transport.rs"]
pub mod transport;
