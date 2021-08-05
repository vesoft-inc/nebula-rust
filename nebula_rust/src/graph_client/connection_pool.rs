/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */

use crate::graph_client::connection::Connection;
use crate::graph_client::pool_config::PoolConfig;
use crate::graph_client::session::Session;

/// The pool of connection to server, it's MT-safe to access.
pub struct ConnectionPool {
    // The connections
    conns: std::sync::Mutex<std::cell::RefCell<std::collections::LinkedList<Connection>>>,
    // It should be immutable
    config: PoolConfig,
    // Address cursor
    cursor: std::sync::atomic::AtomicUsize,
}

impl ConnectionPool {
    /// Construct pool by the configuration
    pub async fn new(conf: &PoolConfig) -> Self {
        let conns = std::collections::LinkedList::<Connection>::new();
        let mut pool = ConnectionPool {
            conns: std::sync::Mutex::new(std::cell::RefCell::new(conns)),
            config: conf.clone(),
            cursor: std::sync::atomic::AtomicUsize::new(0),
        };
        pool.new_connection(pool.config.min_connection_pool_size)
            .await;
        pool
    }

    /// Get a session authenticated by username and password
    /// retry_connect means keep the connection available if true
    pub async fn get_session(
        &self,
        username: &str,
        password: &str,
        retry_connect: bool,
    ) -> std::result::Result<Session<'_>, common::types::ErrorCode> {
        let conn = self.conns.lock().unwrap().borrow_mut().pop_back();
        if let Some(conn) = conn {
            let resp = conn.authenticate(username, password).await?;
            if resp.error_code != common::types::ErrorCode::SUCCEEDED {
                return Err(resp.error_code);
            }
            Ok(Session::new(
                resp.session_id.unwrap(),
                conn,
                self,
                username.to_string(),
                password.to_string(),
                if let Some(time_zone_name) = resp.time_zone_name {
                    std::str::from_utf8(&time_zone_name).unwrap().to_string()
                } else {
                    String::new()
                },
                resp.time_zone_offset_seconds.unwrap(),
                retry_connect,
            ))
        } else {
            Err(common::types::ErrorCode::E_UNKNOWN)
        }
    }

    /// Give back the connection to pool
    #[inline]
    pub fn give_back(&self, conn: Connection) {
        self.conns.lock().unwrap().borrow_mut().push_back(conn);
    }

    /// Get the count of connections
    #[inline]
    pub fn len(&self) -> usize {
        self.conns.lock().unwrap().borrow().len()
    }

    // Add new connection to pool
    async fn new_connection(&mut self, inc: u32) {
        assert!(inc != 0);
        // TODO concurrent these
        for _ in 0..inc {
            let cursor = { self.cursor() };
            match Connection::new_from_address(&self.config.addresses[cursor]).await {
                Ok(conn) => self.conns.lock().unwrap().borrow_mut().push_back(conn),
                Err(_) => (),
            };
        }
    }

    fn cursor(&mut self) -> usize {
        if self.cursor.load(std::sync::atomic::Ordering::Relaxed) >= self.config.addresses.len() {
            self.cursor.store(0, std::sync::atomic::Ordering::Relaxed);
            0
        } else {
            self.cursor
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        }
    }
}
