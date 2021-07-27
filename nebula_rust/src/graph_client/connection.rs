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

pub struct Connection {
    client: client::GraphServiceImpl<
        BinaryProtocol,
        AsyncTransport<TcpStream, transport_response_handler::GraphTransportResponseHandler>,
    >,
}

impl Connection {
    /// Create connection with the specified [host:port]
    pub async fn new(host: &str, port: i32) -> Result<Connection> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr).await?;
        let transport = AsyncTransport::new(
            stream,
            AsyncTransportConfiguration::new(
                transport_response_handler::GraphTransportResponseHandler,
            ),
        );
        Ok(Connection {
            client: client::GraphServiceImpl::new(transport),
        })
    }

    /// Authenticate by username and password
    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> std::result::Result<graph::types::AuthResponse, common::types::ErrorCode> {
        let result = self
            .client
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
    pub async fn signout(
        &self,
        session_id: i64,
    ) -> std::result::Result<(), common::types::ErrorCode> {
        let result = self.client.signout(session_id).await;
        if let Err(_) = result {
            return Err(common::types::ErrorCode::E_RPC_FAILURE);
        }
        Ok(())
    }

    /// Execute the query with current session id which got by authenticating previous
    pub async fn execute(
        &self,
        session_id: i64,
        query: &str,
    ) -> std::result::Result<graph::types::ExecutionResponse, common::types::ErrorCode> {
        let result = self
            .client
            .execute(session_id, &query.to_string().into_bytes())
            .await;
        if let Err(_) = result {
            return Err(common::types::ErrorCode::E_RPC_FAILURE);
        }
        Ok(result.unwrap())
    }
}
