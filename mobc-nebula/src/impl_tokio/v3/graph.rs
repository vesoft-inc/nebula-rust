use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use fbthrift_transport::{
    fbthrift_transport_response_handler::ResponseHandler, AsyncTransport,
    AsyncTransportConfiguration,
};
use mobc::async_trait;
use nebula_client::{
    v3::{GraphClient, GraphSession},
    VersionV3,
};

use super::super::{TokioSleep, TokioTcpStream};
use crate::graph::{GraphClientConfiguration, GraphConnectionManager};

//
pub fn new_graph_connection_manager<H>(
    client_configuration: GraphClientConfiguration,
    transport_configuration: AsyncTransportConfiguration<H>,
) -> GraphConnectionManager<TokioTcpStream, TokioSleep, H, VersionV3>
where
    H: ResponseHandler + Send + Sync + 'static + Unpin,
{
    GraphConnectionManager::new(client_configuration, transport_configuration)
}

//
impl<H> GraphConnectionManager<TokioTcpStream, TokioSleep, H, VersionV3>
where
    H: ResponseHandler + Send + Sync + 'static + Unpin,
{
    async fn get_async_connection(
        &self,
    ) -> Result<GraphSession<AsyncTransport<TokioTcpStream, TokioSleep, H>>, IoError> {
        let transport = AsyncTransport::with_tokio_tcp_connect(
            self.client_configuration.tcp_connect_addr(),
            self.transport_configuration.clone(),
        )
        .await?;
        let client = GraphClient::new(transport);

        let mut session = client
            .authenticate(
                &self.client_configuration.username.as_bytes().to_vec(),
                &self.client_configuration.password.as_bytes().to_vec(),
            )
            .await
            .map_err(|err| IoError::new(IoErrorKind::Other, err))?;

        if let Some(ref space) = self.client_configuration.space {
            session
                .execute(&format!("USE {space}").as_bytes().to_vec())
                .await
                .map_err(|err| IoError::new(IoErrorKind::Other, err))?;
        }

        Ok(session)
    }
}

#[async_trait]
impl<H> mobc::Manager for GraphConnectionManager<TokioTcpStream, TokioSleep, H, VersionV3>
where
    H: ResponseHandler + Send + Sync + 'static + Unpin,
{
    type Connection = GraphSession<AsyncTransport<TokioTcpStream, TokioSleep, H>>;
    type Error = IoError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.get_async_connection().await
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        conn.execute(&"SHOW CHARSET".as_bytes().to_vec())
            .await
            .map_err(|err| IoError::new(IoErrorKind::Other, err))?;

        Ok(conn)
    }

    fn validate(&self, conn: &mut Self::Connection) -> bool {
        !conn.is_close_required()
    }
}
