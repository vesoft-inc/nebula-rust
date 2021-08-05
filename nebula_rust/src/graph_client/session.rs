/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */

use crate::graph_client::connection::Connection;
use crate::graph_client::connection_pool::ConnectionPool;

pub struct Session<'a> {
    session_id: i64,
    conn: Connection,
    pool: &'a ConnectionPool,
    username: String,
    password: String,
    // empty means not a named timezone
    time_zone_name: String,
    // Offset to utc in seconds
    offset_secs: i32,
    // Keep connection if true
    retry_connect: bool,
}

impl<'a> Session<'a> {
    pub fn new(
        session_id: i64,
        conn: Connection,
        pool: &'a ConnectionPool,
        username: String,
        password: String,
        time_zone_name: String,
        offset_secs: i32,
        retry_connect: bool,
    ) -> Self {
        Session {
            session_id: session_id,
            conn: conn,
            pool: pool,
            username: username,
            password: password,
            time_zone_name: time_zone_name,
            offset_secs: offset_secs,
            retry_connect: retry_connect,
        }
    }

    /// sign out the session
    #[inline]
    pub async fn signout(&self) -> std::result::Result<(), common::types::ErrorCode> {
        self.conn.signout(self.session_id).await
    }

    /// Execute the query in current session
    /// The returned error of `Result` only means the request/response status
    /// The error from Nebula Graph is still in `error_code` field in response, so you need check it
    /// to known wether the query execute succeeded
    #[inline]
    pub async fn execute(
        &self,
        query: &str,
    ) -> std::result::Result<graph::types::ExecutionResponse, common::types::ErrorCode> {
        self.conn.execute(self.session_id, query).await
    }

    /// Get the time zone name
    #[inline]
    pub fn time_zone_name(&self) -> &str {
        &self.time_zone_name
    }

    /// Get the time zone offset to UTC in seconds
    #[inline]
    pub fn offset_secs(&self) -> i32 {
        self.offset_secs
    }
}

impl<'a> Drop for Session<'a> {
    /// Drop session will sign out the session in server
    /// and give back connection to pool
    fn drop(&mut self) {
        futures::executor::block_on(self.signout());
        self.pool.give_back(std::mem::take(&mut self.conn));
    }
}
