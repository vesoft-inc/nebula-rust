use core::marker::PhantomData;

use fbthrift_transport::{
    fbthrift_transport_response_handler::ResponseHandler, AsyncTransportConfiguration,
};
use nebula_client::Version;

//
#[derive(Debug, Clone)]
pub struct GraphClientConfiguration {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub space: Option<String>,
}

impl GraphClientConfiguration {
    pub fn new(
        host: String,
        port: u16,
        username: String,
        password: String,
        space: Option<String>,
    ) -> Self {
        Self {
            host,
            port,
            username,
            password,
            space,
        }
    }
}

impl GraphClientConfiguration {
    pub fn tcp_connect_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

//
#[derive(Clone)]
pub struct GraphConnectionManager<S, SLEEP, H, V>
where
    H: ResponseHandler,
    V: Version,
{
    pub client_configuration: GraphClientConfiguration,
    pub transport_configuration: AsyncTransportConfiguration<H>,
    phantom: PhantomData<(S, SLEEP, V)>,
}

impl<S, SLEEP, H, V> GraphConnectionManager<S, SLEEP, H, V>
where
    H: ResponseHandler + Send + Sync + 'static + Unpin,
    V: Version,
{
    pub fn new(
        client_configuration: GraphClientConfiguration,
        transport_configuration: AsyncTransportConfiguration<H>,
    ) -> Self {
        Self {
            client_configuration,
            transport_configuration,
            phantom: PhantomData,
        }
    }
}
