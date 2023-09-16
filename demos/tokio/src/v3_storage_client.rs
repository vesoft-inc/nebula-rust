/*
cargo run -p nebula-demo-tokio --bin nebula_demo_tokio_v3_storage_client 192.168.10.21 9559 9779 
*/

use std::env;
use std::collections::BTreeMap;

use fbthrift_transport::{AsyncTransport, AsyncTransportConfiguration};
use nebula_client::v3::storage::{StorageClient, StorageTransportResponseHandler};
use nebula_client::v3::meta::{MetaClient, MetaTransportResponseHandler};
use nebula_fbthrift_storage_v3::types::{ScanCursor,ScanEdgeRequest,ScanVertexRequest,VertexProp,EdgeProp};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {


    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or_else(|_| "127.0.0.1".to_owned()));
    let mport: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("MPORT").unwrap_or_else(|_| "9559".to_owned()))
        .parse()
        .unwrap();
    let sport: u16 = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("SPORT").unwrap_or_else(|_| "9779".to_owned()))
        .parse()
        .unwrap();
    let vspace_name = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("VSPACE").unwrap_or_else(|_| "NONE".to_owned()));
    let tag_name = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("TAG").unwrap_or_else(|_| "NONE".to_owned()));
    let espace_name = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("ESAPCE").unwrap_or_else(|_| "NONE".to_owned()));
    let edge_name = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("EDGE").unwrap_or_else(|_| "NONE".to_owned()));

    println!("v3_meta_client {domain} {mport}",);
    println!("v3_storage_client {domain} {sport}",);

        
    let maddr = format!("{domain}:{mport}");
    let saddr = format!("{domain}:{sport}");

    
    let mtransport = AsyncTransport::with_tokio_tcp_connect(
        maddr,
        AsyncTransportConfiguration::new(MetaTransportResponseHandler),
    )
    .await?;

    let stransport = AsyncTransport::with_tokio_tcp_connect(
        saddr,
        AsyncTransportConfiguration::new(StorageTransportResponseHandler),
    )
    .await?;
    

    // 创建meta_client
    let mclient = MetaClient::new(mtransport);




    

    

    // 创建storage_client
    let sclient = StorageClient::new(stransport);


    //创建scan_vertex_request

    let mut part: BTreeMap<i32, ScanCursor> = BTreeMap::new();

    for partition_id in 1..=10 {
        let cursor = ScanCursor {
            next_cursor:  None, // Option为空
            ..Default::default()
          
        };
        part.insert(partition_id, cursor);
    }

    let mut part4: BTreeMap<i32, ScanCursor> = BTreeMap::new(); 
    let cursor = ScanCursor {
        next_cursor:  None, // Option为空
        ..Default::default()
      
    };
    part4.insert(1, cursor);

    let column: Vec<VertexProp> = vec![
        VertexProp {
            tag: 2, // 设置tag字段的值
            props: vec![ // 创建一个包含多个字节串的props字段
                b"_vid".to_vec(),
                b"name".to_vec(),
                b"age".to_vec(),
            ],
            ..Default::default()
        }
    ];

    let scan_vertex_request = ScanVertexRequest {
        space_id: 1, 
        parts: part4, 
        return_columns: column, 
        limit: 10000, 
        start_time: Some(0), 
        end_time: Some(922337203685477580),  
        filter: None, 
        only_latest_version: false, 
        enable_read_from_follower: true, 
        common: None, 
        ..Default::default()
        
    };

    //创建scan_edge_request
    let mut part2: BTreeMap<i32, ScanCursor> = BTreeMap::new();

    for partition_id in 1..=10 {
        let cursor = ScanCursor {
            next_cursor:  None, // Option为空
            ..Default::default()
          
        };
        part2.insert(partition_id, cursor);
    }

    let mut part3: BTreeMap<i32, ScanCursor> = BTreeMap::new(); 
    let cursor = ScanCursor {
        next_cursor:  None, // Option为空
        ..Default::default()
      
    };
    part3.insert(1, cursor);

    let edge_prop = EdgeProp {
        r#type: 4, // 用适当的值替换 EdgeType 类型
        props: vec![
            b"_src".to_vec(),
            b"_type".to_vec(),
            b"_rank".to_vec(),
            b"_dst".to_vec(),
            b"start_year".to_vec(),
            b"end_year".to_vec(),
        ], // 设置 props 字段的值
        ..Default::default() // 这个字段可以使用 Default::default() 初始化
    };

    let scan_edge_request = ScanEdgeRequest {
        space_id: 1,  
        parts: part3,  
        return_columns: vec![edge_prop],  
        limit: 1000,  
        start_time: Some(0),  
        end_time: Some(922337203685477580),    
        filter: None,     
        only_latest_version: false,  
        enable_read_from_follower: true,  
        common: None, 
        ..Default::default()
    };
   


    let res1 = sclient.scan_vertex(&scan_vertex_request).await?;
    let res2 = sclient.scan_edge(&scan_edge_request).await?;

    println!("{res1:?}");
    println!("{res2:?}");


    Ok(())
}
