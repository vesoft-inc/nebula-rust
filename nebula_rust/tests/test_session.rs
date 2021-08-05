/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */

extern crate nebula_rust;

#[cfg(test)]
mod test_connection {
    use nebula_rust::graph_client;
    use nebula_rust::value::data_set::DataSetValue;
    use nebula_rust::value::row::RowValue;

    #[tokio::test]
    async fn basic_op() {
        let mut conf = graph_client::pool_config::PoolConfig::new();
        conf.min_connection_pool_size(2)
            .max_connection_pool_size(10)
            .address("localhost:9669".to_string());

        let pool = graph_client::connection_pool::ConnectionPool::new(&conf).await;
        let session = pool.get_session("root", "nebula", true).await.unwrap();

        let resp = session.execute("YIELD 1").await.unwrap();
        assert!(resp.error_code == common::types::ErrorCode::SUCCEEDED);

        let mut dt = common::types::DataSet::new(&["1".to_string()]);
        dt.push(common::types::Row::new(&[common::types::Value::iVal(1)]));
        assert!(dt == resp.data.unwrap());
    }
}
