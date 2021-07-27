#[cfg(feature = "futures_io")]
#[path = "futures_io.rs"]
pub mod futures_io;

#[cfg(feature = "tokio02_io")]
#[path = "tokio02_io.rs"]
pub mod tokio02_io;

#[cfg(feature = "tokio_io")]
#[path = "tokio_io.rs"]
pub mod tokio_io;

//
//
//
#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use self::futures_io::transport;
#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use self::futures_io::transport::AsyncTransport;

#[cfg(all(
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use self::tokio02_io::transport;
#[cfg(all(
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use self::tokio02_io::transport::AsyncTransport;

#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use self::tokio_io::transport;
#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use self::tokio_io::transport::AsyncTransport;

//
//
//
pub mod configuration;
pub use configuration::{AsyncTransportConfiguration, DefaultAsyncTransportConfiguration};

pub use fbthrift_transport_response_handler;
