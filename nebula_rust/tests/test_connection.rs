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
        let result = graph_client::connection::Connection::new("localhost", 9669).await;
        assert!(result.is_ok());
        let conn = result.unwrap();

        let result = conn.authenticate("root", "nebula").await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.error_code == common::types::ErrorCode::SUCCEEDED);
        let session_id = response.session_id.unwrap();

        let result = conn.execute(session_id, "YIELD 1").await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.error_code == common::types::ErrorCode::SUCCEEDED);
        let mut dt = common::types::DataSet::new(&["1".to_string()]);
        dt.push(common::types::Row::new(&[common::types::Value::iVal(1)]));
        assert!(dt == response.data.unwrap());

        let result = conn.signout(session_id).await;
        assert!(result.is_ok());
    }
}
