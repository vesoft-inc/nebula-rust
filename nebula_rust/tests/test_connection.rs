/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License.
 */

extern crate nebula_rust;

#[cfg(test)]
mod test_connection {
    use nebula_rust::graph_client;

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
        println!("{:?}", response.data.unwrap());

        let result = conn.signout(session_id).await;
        assert!(result.is_ok());
    }
}
