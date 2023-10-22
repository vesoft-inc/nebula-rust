use core::time::Duration;

use async_trait::async_trait;


use nebula_fbthrift_storage::v3::{
    errors::graph_storage_service::{ScanEdgeError, ScanVertexError},
    types::{ScanEdgeRequest,ScanVertexRequest,ScanResponse},
};

use serde::de::DeserializeOwned;
use deserialize_nebula_fbthrift::v3::de::{
    data::DataDeserializeError, deserialize_scan_struct_response,
};

#[async_trait]
pub trait StorageQuery {
    #[allow(clippy::ptr_arg)]
    async fn query_vertex<D: DeserializeOwned>(
        &mut self,
        req: &ScanVertexRequest,
    ) -> Result<StorageQueryOutput<D>, StorageQueryError>;

    async fn query_edge<D: DeserializeOwned>(
        &mut self,
        req: &ScanEdgeRequest,
    ) -> Result<StorageQueryOutput<D>, StorageQueryError>;
 
    async fn show_vertexs(&mut self,req: &ScanVertexRequest) -> Result<StorageQueryOutput<Vertex>, StorageQueryError> {
        self.query_vertex(req).await
    }
    
    async fn show_edges(&mut self,req: &ScanEdgeRequest) -> Result<StorageQueryOutput<Edge>, StorageQueryError> {
        self.query_edge(req).await
    }   


}

#[derive(Debug)]
pub struct StorageQueryOutput<D>
where
    D: DeserializeOwned,
{
    pub latency: Duration,
    pub data_set: Vec<D>,
}

impl<D> StorageQueryOutput<D>
where
    D: DeserializeOwned,
{
    pub fn new(res: ScanResponse) -> Result<Self, StorageQueryError> {
        let latency = Duration::from_micros(res.result.latency_in_us as u64);
        let data_set = deserialize_scan_struct_response::<D>(&res)
            .map_err(StorageQueryError::DataDeserializeError)?;

        Ok(Self {
            latency,
            data_set,
        })
    }
}

//
//
//
#[derive(Debug)]
pub enum StorageQueryError {
    ScanEdgeError(ScanEdgeError),
    ScanVertexError(ScanVertexError),
    DataDeserializeError(DataDeserializeError),
}

impl core::fmt::Display for StorageQueryError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ScanEdgeError(err) => write!(f, "ScanEdgeError {err}"),
            Self::ScanVertexError(err) => write!(f, "ScanVertexError {err}"),
            Self::DataDeserializeError(err) => write!(f, "DataDeserializeError {err}"),
        }
    }
}

impl std::error::Error for StorageQueryError {
    fn description(&self) -> &str {
        match self {
            Self::ScanVertexError(_) => "ScanVertexError",
            Self::ScanEdgeError(_) => "ScanEdgeError",
            Self::DataDeserializeError(_) => "DataDeserializeError",
        }
    }
}

//
//
//
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Vertex {
    #[serde(rename(deserialize = "_vid"))]
    pub _vid: String,
    #[serde(rename(deserialize = "player._vid"))]
    pub player: String,
    #[serde(rename(deserialize = "player.name"))]
    pub name: String,
    #[serde(rename(deserialize = "player.age"))]
    pub age: i64,
}





#[derive(Deserialize, Debug)]
pub struct Edge {
    #[serde(rename(deserialize = "serve._src"))]
    pub _src: String,
    #[serde(rename(deserialize = "serve._type"))]
    pub _type: i64,
    #[serde(rename(deserialize = "serve._rank"))]
    pub rank: i64,
    #[serde(rename(deserialize = "serve._dst"))]
    pub _dst: String,
    #[serde(rename(deserialize = "serve.start_year"))]
    pub start_year: i64,
    #[serde(rename(deserialize = "serve.end_year"))]
    pub end_year: i64,
}


