use async_trait::async_trait;
use bytes::Bytes;
use fbthrift::{BinaryProtocol, Transport};
use nebula_fbthrift_storage::v3::{
    client::{GraphStorageService, GraphStorageServiceImpl},
    errors::graph_storage_service::{ScanEdgeError, ScanVertexError},
    types::{ScanEdgeRequest, ScanResponse, ScanVertexRequest},
};
use serde::de::DeserializeOwned;
use super::scan_struct::{StorageQuery, StorageQueryError, StorageQueryOutput};
//
//
//
struct StorageConnection<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    service: GraphStorageServiceImpl<BinaryProtocol, T>,
}

impl<T> StorageConnection<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    fn new(transport: T) -> Self {
        Self {
            service: GraphStorageServiceImpl::<BinaryProtocol, _>::new(transport),
        }
    }
}

//
//
//
pub struct StorageClient<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    connection: StorageConnection<T>,
}

impl<T> StorageClient<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    pub fn new(transport: T) -> Self {
        Self {
            connection: StorageConnection::new(transport),
        }
    }

    // mclient: &MetaClient<AsyncTransport<Compat<TcpStream>, Sleep, MetaTransportResponseHandler>>,
    pub async fn scan_vertex(
        &self,
        req: &ScanVertexRequest,
    ) -> Result<ScanResponse, ScanVertexError> {
        let res = self.connection.service.scanVertex(req).await?;
        Ok(res)
    }

    pub async fn scan_edge(
        &self,
        req: &ScanEdgeRequest,
    ) -> Result<ScanResponse, ScanEdgeError> {
        let res = self.connection.service.scanEdge(req).await?;

        Ok(res)
    }
}

#[async_trait]
impl<T> StorageQuery for StorageClient<T>
where
    T: Transport + Send + Sync,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    async fn query_vertex<D: DeserializeOwned>(
        &mut self,
        req: &ScanVertexRequest,
    ) -> Result<StorageQueryOutput<D>, StorageQueryError> {
        let res = self
            .scan_vertex(req)
            .await
            .map_err(StorageQueryError::ScanVertexError)?;


        StorageQueryOutput::new(res)
    }

    async fn query_edge<D: DeserializeOwned>(
        &mut self,
        req: &ScanEdgeRequest,
    ) -> Result<StorageQueryOutput<D>, StorageQueryError> {
        let res = self
            .scan_edge(req)
            .await
            .map_err(StorageQueryError::ScanEdgeError)?;


        StorageQueryOutput::new(res)
    }
}
