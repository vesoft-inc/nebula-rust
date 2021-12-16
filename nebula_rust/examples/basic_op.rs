/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */

use nebula_rust::graph_client;

#[tokio::main]
async fn main() {
    let mut conf = graph_client::pool_config::PoolConfig::new();
    conf.min_connection_pool_size(2)
        .max_connection_pool_size(10)
        .address("localhost:9669".to_string());

    let pool = graph_client::connection_pool::ConnectionPool::new(&conf).await;
    let session = pool.get_session("root", "nebula", true).await.unwrap();

    let resp = session.execute("YIELD 1").await.unwrap();
    assert!(resp.error_code == common::types::ErrorCode::SUCCEEDED);

    println!("{:?}", resp.data.as_ref().unwrap());
    println!(
        "The result of query `YIELD 1' is {}.",
        if let common::types::Value::iVal(v) = resp.data.unwrap().rows[0].values[0] {
            v
        } else {
            panic!()
        }
    );
}
