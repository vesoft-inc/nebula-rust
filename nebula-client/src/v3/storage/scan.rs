// cargo run -p nebula-client --bin nebula_demo 192.168.10.21 9559 9779     
use std::collections::BTreeMap;

use crate::v3::meta::{MetaClient, MetaTransportResponseHandler};
use super::{StorageClient,StorageTransportResponseHandler};
use fbthrift_transport::{AsyncTransport, AsyncTransportConfiguration};
use nebula_fbthrift_storage_v3::{
    errors::graph_storage_service::{ScanEdgeError, ScanVertexError},
    types::{ScanCursor,ScanEdgeRequest,ScanVertexRequest,VertexProp,EdgeProp},
};
use deserialize_nebula_fbthrift_storage::v3::de::{deserialize_scan_response,datadeal::ProcessError};
use nebula_fbthrift_meta_v3::{
    errors::meta_service::{GetSpaceError, ListPartsError, ListTagsError,ListEdgesError,GetPartsAllocError},
    types::{TagItem,EdgeItem,ListPartsResp},
};
use std::error::Error;
use std::fmt;

const DEFAULT_START_TIME: i64 = 0;
const DEFAULT_END_TIME: i64 = i64::MAX;
const DEFAULT_LIMIT: i64 = 1000;

pub async fn scan_vertex(maddr:String,vspace_name:String,tag_name:String) -> Result<Vec<Vec<String>>, ScanError> {



    let mtransport = AsyncTransport::with_tokio_tcp_connect(
        maddr,
        AsyncTransportConfiguration::new(MetaTransportResponseHandler),
    )
    .await.map_err(|e| ScanError::Custom(format!("Meta Transport Error: {}", e)))?;

    // 创建meta_client
    let mclient = MetaClient::new(mtransport);

    // 获取所需信息
    let space_name: Vec<u8> = vspace_name.into_bytes();
    let space_id_res = mclient.get_space(space_name).await.map_err( ScanError::GetSpaceError)?;
    let space_id = space_id_res.item.space_id;

    let tag_item_res = mclient.list_tags(space_id).await.map_err( ScanError::ListTagsError)?;
    let ress = create_column(&tag_item_res.tags, tag_name).await?;

    let part_res = mclient.get_parts(space_id).await.map_err( ScanError::GetPartsAllocError)?;
    let mut parts_id:Vec<i32> = Vec::new();
    for part_id in part_res.parts.keys() {
        parts_id.push(*part_id);
    }
    let part_id_res = mclient.list_parts(space_id,parts_id.clone()).await.map_err( ScanError::ListPartsError)?;
    let result_map = get_leader_map(&part_id_res, &parts_id).await;
    

    
    let mut new_column: Vec<VertexProp> = Vec::new();

    match ress {
        Some(prop) => {
            // 尝试将 prop 转换为 VertexProp
            if let Prop::VertexProp(vertex_prop) = prop {
                // 如果转换成功，将 vertex_prop 插入到 new_column 中
                new_column.push(vertex_prop.clone()); // 或者使用引用，取决于你的需求
            } else {
                eprintln!("Error: VertexProp is None");
                // 或者执行其他操作
            }
        }
        None => {
            eprintln!("Error: VertexProp is None");
            // 或者执行其他操作
        }
    }

    let mut data_set:Vec<Vec<String>> = Vec::new();
    
    for (part_id, leader) in result_map {
        println!("Part ID: {}, Leader: {}", part_id, leader);
        let saddr = leader;
    
        let stransport = AsyncTransport::with_tokio_tcp_connect(
            saddr,
            AsyncTransportConfiguration::new(StorageTransportResponseHandler),
        )
        .await.map_err(|e| ScanError::Custom(format!("Storage Transport Error: {}", e)))?;
    
        // 创建storage_client
        let sclient = StorageClient::new(stransport);
        
        //创建scan_vertex_request
        let cursor = ScanCursor {
            next_cursor:  None, // Option为空
            ..Default::default()
          
        };

        let mut part: BTreeMap<i32, ScanCursor> = BTreeMap::new();
        part.insert(part_id, cursor);

        
        let scan_vertex_request = ScanVertexRequest {
            space_id: space_id, 
            parts: part, 
            return_columns: new_column.clone(), 
            limit: DEFAULT_LIMIT, 
            start_time: Some(DEFAULT_START_TIME), 
            end_time: Some(DEFAULT_END_TIME),  
            filter: None, 
            only_latest_version: false, 
            enable_read_from_follower: true, 
            common: None, 
            ..Default::default()
            
        }; 
    
    
        let res1 = sclient.scan_vertex(&scan_vertex_request).await.map_err( ScanError::ScanVertexError)?;
     
        let part_vertex_result : Vec<String> = deserialize_scan_response(&res1,true).map_err( ScanError::DeserializeVertexError)?;
    
        data_set.push(part_vertex_result);
    }


    Ok(data_set)
}





pub async fn scan_edge(maddr:String,espace_name:String,edge_name:String) -> Result<Vec<Vec<String>>, ScanError> {

    
    
    let mtransport = AsyncTransport::with_tokio_tcp_connect(
        maddr,
        AsyncTransportConfiguration::new(MetaTransportResponseHandler),
    )
    .await.map_err(|e| ScanError::Custom(format!("Meta Transport Error: {}", e)))?;


    // 创建meta_client
    let mclient = MetaClient::new(mtransport);

    // 获取所需信息
    let space_name: Vec<u8> = espace_name.into_bytes();
    let space_id_res = mclient.get_space(space_name).await.map_err( ScanError::GetSpaceError)?;
    let space_id = space_id_res.item.space_id;

    let edge_item_res = mclient.list_edges(space_id).await.map_err( ScanError::ListEdgesError)?;
    let ress = create_column(&edge_item_res.edges, edge_name).await?;

    let part_res = mclient.get_parts(space_id).await.map_err( ScanError::GetPartsAllocError)?;
    let mut parts_id:Vec<i32> = Vec::new();
    for part_id in part_res.parts.keys() {
        parts_id.push(*part_id);
    }
    let part_id_res = mclient.list_parts(space_id,parts_id.clone()).await.map_err( ScanError::ListPartsError)?;
    let result_map = get_leader_map(&part_id_res, &parts_id).await;
    
    let mut new_column: Vec<EdgeProp> = Vec::new();

    match ress {
        Some(prop) => {
            // 尝试将 prop 转换为 VertexProp
            if let Prop::EdgeProp(edge_prop) = prop {
                // 如果转换成功，将 vertex_prop 插入到 new_column 中
                new_column.push(edge_prop.clone()); // 或者使用引用，取决于你的需求
            } else {
                eprintln!("Error: VertexProp is None");
                // 或者执行其他操作
            }
        }
        None => {
            eprintln!("Error: VertexProp is None");
            // 或者执行其他操作
        }
    }

    let mut data_set:Vec<Vec<String>> = Vec::new();

    for (part_id, leader) in result_map {
        println!("Part ID: {}, Leader: {}", part_id, leader);
        let saddr = leader;
    
        let stransport = AsyncTransport::with_tokio_tcp_connect(
            saddr,
            AsyncTransportConfiguration::new(StorageTransportResponseHandler),
        )
        .await.map_err(|e| ScanError::Custom(format!("Storage Transport Error: {}", e)))?;
    
        // 创建storage_client
        let sclient = StorageClient::new(stransport);
        
        //创建scan_vertex_request
        let cursor = ScanCursor {
            next_cursor:  None, // Option为空
            ..Default::default()
          
        };

        let mut part: BTreeMap<i32, ScanCursor> = BTreeMap::new();
        part.insert(part_id, cursor);


        let scan_edge_request = ScanEdgeRequest {
            space_id: space_id,  
            parts: part,  
            return_columns: new_column.clone(),  
            limit: DEFAULT_LIMIT,  
            start_time: Some(DEFAULT_START_TIME),  
            end_time: Some(DEFAULT_END_TIME),    
            filter: None,     
            only_latest_version: false,  
            enable_read_from_follower: true,  
            common: None, 
            ..Default::default()
        };
    

    
        let res2 = sclient.scan_edge(&scan_edge_request).await.map_err( ScanError::ScanEdgeError)?;

        
    
        let part_edge_result : Vec<String> = deserialize_scan_response(&res2,false).map_err( ScanError::DeserializeEdgeError)?;
    
        data_set.push(part_edge_result);
    }


    Ok(data_set)
}

pub trait CommonProp {
    fn get_props(&self) -> Vec<Vec<u8>>;
    fn get_name(&self) -> Vec<u8>;
    fn get_id(&self) -> i32;
    fn get_type(&self) -> i32;
    fn is_tag(&self) -> bool;
}

impl CommonProp for TagItem {
    fn get_props(&self) -> Vec<Vec<u8>> {
        let mut props: Vec<Vec<u8>> = vec![b"_vid".to_vec()];

        for column_def in &self.schema.columns {
            props.push(column_def.name.clone());
        }

        props
    }
    fn get_name(&self) -> Vec<u8> {
        self.tag_name.clone()
    }

    fn get_id(&self) -> i32 {
        self.tag_id
    }

    fn get_type(&self) -> i32 {
        unimplemented!(); // 标记为未实现
    }

    fn is_tag(&self) -> bool {
        true
    }
}

impl CommonProp for EdgeItem {
    fn get_props(&self) -> Vec<Vec<u8>> {
        let mut props: Vec<Vec<u8>> = vec![b"_src".to_vec(), b"_type".to_vec(), b"_rank".to_vec(), b"_dst".to_vec()];

        for column_def in &self.schema.columns {
            props.push(column_def.name.clone());
        }

        props
    }
    fn get_name(&self) -> Vec<u8>  {
        self.edge_name.clone()
    }

    fn get_id(&self) -> i32 {
        unimplemented!(); // 标记为未实现
    }

    fn get_type(&self) -> i32 {
        // 在EdgeItem中可能需要某个字段来表示类型，用于返回类型
        // 例如：self.edge_type
        self.edge_type
    }

    fn is_tag(&self) -> bool {
        false
    }
}


#[derive(Debug)]
pub enum Prop {
    VertexProp(VertexProp),
    EdgeProp(EdgeProp),
}


pub async fn create_column<T>(
    items: &Vec<T>,
    target_name: String,
) -> Result<Option<Prop>, ScanError>
where
    T: CommonProp + 'static,
{
    for item in items.iter() {
        let name_str = String::from_utf8(item.get_name().clone())
        .map_err(|e| ScanError::Custom(format!("Meta Transport Error: {}", e)))?;
        if name_str == target_name {
            let props = item.get_props();

            if item.is_tag() {
                let ver: VertexProp = VertexProp {
                    tag: item.get_id(),
                    props,
                    ..Default::default()
                };

                return Ok(Some(Prop::VertexProp(ver)));
            } else {
                let edg: EdgeProp = EdgeProp {
                    r#type: item.get_type(),
                    props,
                    ..Default::default()
                };

                return Ok(Some(Prop::EdgeProp(edg)));
            }
        }
    }

    Err(ScanError::CreateColumnError)
}




pub async fn get_leader_map(part_id_res: &ListPartsResp, parts_id: &Vec<i32>) -> BTreeMap<i32, String> {
    let mut leader_map = BTreeMap::new();

    for part_id in parts_id {
        for part in &part_id_res.parts {
            if part.part_id == *part_id {
                if let Some(leader) = &part.leader {
                    let leader_str = format!("{}:{}", leader.host, leader.port);
                    leader_map.insert(*part_id, leader_str);
                }
                break; // 找到匹配的 part_id，不再继续循环
            }
        }
    }

    leader_map
}

#[derive(Debug)]
pub enum ScanError {
    DeserializeVertexError(ProcessError),
    DeserializeEdgeError(ProcessError),
    ScanVertexError(ScanVertexError),
    ScanEdgeError(ScanEdgeError),
    GetSpaceError(GetSpaceError),
    ListTagsError(ListTagsError),
    ListEdgesError(ListEdgesError),
    GetPartsAllocError(GetPartsAllocError),
    ListPartsError(ListPartsError),
    CreateColumnError,
    GetLeaderMapError,
    Custom(String),
}

impl Error for ScanError {}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DeserializeVertexError(err) => write!(f, "Deserialize Vertex Error: {}", err),
            Self::DeserializeEdgeError(err) => write!(f, "Deserialize Edge Error: {}", err),
            Self::ScanVertexError(err) => write!(f, "Scan Vertex Error: {}", err),
            Self::ScanEdgeError(err) => write!(f, "Scan Edge Error: {}", err),
            Self::GetSpaceError(err) => write!(f, "Get Space Error: {}", err),
            Self::ListTagsError(err) => write!(f, "List Tags Error: {}", err),
            Self::ListEdgesError(err) => write!(f, "List Edges Error: {}", err),
            Self::GetPartsAllocError(err) => write!(f, "Get Parts Allocation Error: {}", err),
            Self::ListPartsError(err) => write!(f, "List Parts Error: {}", err),
            Self::CreateColumnError => write!(f, "Create Column Error"),
            Self::GetLeaderMapError => write!(f, "Get Leader Map Error"),
            Self::Custom(msg) => write!(f, "Custom Error: {}", msg),
        }
    }
}
