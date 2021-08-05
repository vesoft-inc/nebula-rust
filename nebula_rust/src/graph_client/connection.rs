/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */
use std::io::Result;

use fbthrift::BinaryProtocol;
use fbthrift_transport::{tokio_io::transport::AsyncTransport, AsyncTransportConfiguration};
use graph::client;
use graph::client::GraphService;
use tokio::net::TcpStream;

use crate::graph_client::transport_response_handler;

/// The simple abstraction of a connection to nebula graph server
#[derive(Default)]
pub struct Connection {
    // The option is used to construct a null connection
    // which is used to give back the connection to pool from session
    // So we could assume it's alway not null
    client: Option<
        client::GraphServiceImpl<
            BinaryProtocol,
            AsyncTransport<TcpStream, transport_response_handler::GraphTransportResponseHandler>,
        >,
    >,
}

impl Connection {
    /// Create connection with the specified [host:port] address
    pub async fn new_from_address(address: &str) -> Result<Connection> {
        let stream = TcpStream::connect(address).await?;
        let transport = AsyncTransport::new(
            stream,
            AsyncTransportConfiguration::new(
                transport_response_handler::GraphTransportResponseHandler,
            ),
        );
        Ok(Connection {
            client: Some(client::GraphServiceImpl::new(transport)),
        })
    }

    /// Create connection with the specified [host:port]
    pub async fn new(host: &str, port: i32) -> Result<Connection> {
        let address = format!("{}:{}", host, port);
        Connection::new_from_address(&address).await
    }

    /// Authenticate by username and password
    /// The returned error of `Result` only means the request/response status
    /// The error from Nebula Graph is still in `error_code` field in response, so you need check it
    /// to known wether authenticate succeeded
    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> std::result::Result<graph::types::AuthResponse, common::types::ErrorCode> {
        let result = self
            .client
            .as_ref()
            .unwrap()
            .authenticate(
                &username.to_string().into_bytes(),
                &password.to_string().into_bytes(),
            )
            .await;
        if let Err(_) = result {
            return Err(common::types::ErrorCode::E_RPC_FAILURE);
        }
        Ok(result.unwrap())
    }

    /// Sign out the authentication by session id which got by authenticating previous
    /// The returned error of `Result` only means the request/response status
    pub async fn signout(
        &self,
        session_id: i64,
    ) -> std::result::Result<(), common::types::ErrorCode> {
        let result = self.client.as_ref().unwrap().signout(session_id).await;
        if let Err(_) = result {
            return Err(common::types::ErrorCode::E_RPC_FAILURE);
        }
        Ok(())
    }

    /// Execute the query with current session id which got by authenticating previous
    /// The returned error of `Result` only means the request/response status
    /// The error from Nebula Graph is still in `error_code` field in response, so you need check it
    /// to known wether the query execute succeeded
    pub async fn execute(
        &self,
        session_id: i64,
        query: &str,
    ) -> std::result::Result<graph::types::ExecutionResponse, common::types::ErrorCode> {
        let result = self
            .client
            .as_ref()
            .unwrap()
            .execute(session_id, &query.to_string().into_bytes())
            .await;
        if let Err(_) = result {
            return Err(common::types::ErrorCode::E_RPC_FAILURE);
        }
        Ok(result.unwrap())
    }
}
