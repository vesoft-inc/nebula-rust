use bytes::Bytes;
use fbthrift::{BinaryProtocol, Transport};
use nebula_fbthrift_storage_v1::{
    client::{StorageService, StorageServiceImpl},
    errors::storage_service::{ScanEdgeError, ScanVertexError},
    types::{ScanEdgeRequest, ScanEdgeResponse, ScanVertexRequest, ScanVertexResponse},
};

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
    service: StorageServiceImpl<BinaryProtocol, T>,
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
            service: StorageServiceImpl::<BinaryProtocol, _>::new(transport),
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

    pub async fn scan_vertex(
        &self,
        req: &ScanVertexRequest,
    ) -> Result<ScanVertexResponse, ScanVertexError> {
        let res = self.connection.service.scanVertex(req).await?;

        Ok(res)
    }

    pub async fn scan_edge(
        &self,
        req: &ScanEdgeRequest,
    ) -> Result<ScanEdgeResponse, ScanEdgeError> {
        let res = self.connection.service.scanEdge(req).await?;

        Ok(res)
    }
}
