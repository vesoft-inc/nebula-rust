/*
cargo run -p nebula-demo-tokio --bin nebula_demo_scan 192.168.10.21:9559,192.168.10.22:9559,192.168.10.23:9559 basketballplayer player basketballplayer serve
*/

use std::env;
use nebula_client::v3::storage::{scan_vertex,scan_edge};




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {


    let maddr = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or_else(|_| "127.0.0.1:9559".to_owned()));
    let vspace_name = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("VSPACE").unwrap_or_else(|_| "NONE".to_owned()));
    let tag_name = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("TAG").unwrap_or_else(|_| "NONE".to_owned()));
    let espace_name = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("ESAPCE").unwrap_or_else(|_| "NONE".to_owned()));
    let edge_name = env::args()
        .nth(5)
        .unwrap_or_else(|| env::var("EDGE").unwrap_or_else(|_| "NONE".to_owned()));

    println!("v3_meta_client {maddr}",);

        

    // 检查 vspace_name 和 tag_name 是否为空，如果为空就不调用 ScanVertex 函数
    if vspace_name != "NONE" || tag_name != "NONE" {
        let _vertex_data_set = scan_vertex(maddr.clone(), vspace_name, tag_name).await?;
        // print!("{:?}",_vertex_data_set);
    }

        // 检查 vspace_name 和 tag_name 是否为空，如果为空就不调用 ScanVertex 函数
    if espace_name != "NONE" || edge_name != "NONE" {
        let _edge_data_set = scan_edge(maddr.clone(), espace_name,  edge_name).await?;
        // print!("{:?}",_edge_data_set);
    }
    


    Ok(())
}
