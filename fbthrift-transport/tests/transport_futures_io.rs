#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
mod transport_futures_io_tests {
    use std::io;
    use std::net::{TcpListener, TcpStream};
    use std::sync::Arc;
    use std::thread;

    use async_executor::{Executor, Task};
    use async_io::Async;
    use bytes::Bytes;
    use fbthrift::Transport;
    use futures_lite::future::{self, block_on};
    use futures_lite::io::{AsyncReadExt, AsyncWriteExt};

    use fbthrift_transport::AsyncTransport;
    use fbthrift_transport_response_handler::ResponseHandler;

    #[derive(Clone)]
    pub struct FooResponseHandler;

    impl ResponseHandler for FooResponseHandler {
        fn try_make_static_response_bytes(
            &mut self,
            _request_bytes: &[u8],
        ) -> io::Result<Option<Vec<u8>>> {
            Ok(None)
        }

        fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>> {
            Ok(if response_bytes == b"abcde" {
                Some(5)
            } else {
                None
            })
        }
    }

    #[test]
    fn simple() -> io::Result<()> {
        let ex = Executor::new();
        let ex = Arc::new(ex);

        let ex_with_run_pending = ex.clone();
        thread::spawn(move || block_on(ex_with_run_pending.run(future::pending::<()>())));

        block_on(async move {
            let listen_addr_for_server = TcpListener::bind("127.0.0.1:0")
                .unwrap()
                .local_addr()
                .unwrap();
            let listen_addr_for_client = listen_addr_for_server.clone();

            let server: Task<io::Result<()>> = ex.clone().spawn(async move {
                let listener = Async::<TcpListener>::bind(listen_addr_for_server)?;

                let (mut stream, _) = listener.accept().await?;

                let mut n: usize = 0;
                let mut buf = vec![0; 5];
                loop {
                    stream.read_exact(&mut buf).await?;
                    stream.write_all(&buf).await?;

                    n += 1;
                    if n >= 10 {
                        break;
                    }
                }

                Ok(())
            });

            let client: Task<io::Result<()>> = ex.clone().spawn(async move {
                let stream = Async::<TcpStream>::connect(listen_addr_for_client).await?;

                let transport = AsyncTransport::with_default_configuration(stream);

                for n in 0..10_usize {
                    let cursor = transport
                        .call(Bytes::from("abcde"))
                        .await
                        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

                    println!("futures_io transport.call {} {:?}", n, cursor);
                    assert_eq!(cursor.into_inner(), Bytes::from("abcde"));
                }

                Ok(())
            });

            client.await?;
            server.cancel().await;

            Ok(())
        })
    }
}
