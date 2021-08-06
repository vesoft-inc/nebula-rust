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
    /// The connections
    /// The interior mutable to enable could get multiple sessions in one scope
    conns: std::sync::Mutex<std::cell::RefCell<std::collections::LinkedList<Connection>>>,
    /// It should be immutable
    config: PoolConfig,
    /// Address cursor
    cursor: std::cell::RefCell<std::sync::atomic::AtomicUsize>,
    /// The total count of connections, contains which hold by session
    conns_count: std::cell::RefCell<std::sync::atomic::AtomicUsize>,
}

impl ConnectionPool {
    /// Construct pool by the configuration
    pub async fn new(conf: &PoolConfig) -> Self {
        let conns = std::collections::LinkedList::<Connection>::new();
        let pool = ConnectionPool {
            conns: std::sync::Mutex::new(std::cell::RefCell::new(conns)),
            config: conf.clone(),
            cursor: std::cell::RefCell::new(std::sync::atomic::AtomicUsize::new(0)),
            conns_count: std::cell::RefCell::new(std::sync::atomic::AtomicUsize::new(0)),
        };
        assert!(pool.config.min_connection_pool_size <= pool.config.max_connection_pool_size);
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
        if self.conns.lock().unwrap().borrow_mut().is_empty() {
            self.new_connection(1).await;
        }
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
    // inc is the count of new connection created, which shouldn't be zero
    // the incremental count maybe can't fit when occurs error in connection creating
    async fn new_connection(&self, inc: u32) {
        assert!(inc != 0);
        // TODO concurrent these
        let mut count = 0;
        let mut loop_count = 0;
        let loop_limit = inc as usize * self.config.addresses.len();
        while count < inc {
            if count as usize
                + self
                    .conns_count
                    .borrow()
                    .load(std::sync::atomic::Ordering::Acquire)
                >= self.config.max_connection_pool_size as usize
            {
                // Reach the pool size limit
                break;
            }
            let cursor = { self.cursor() };
            match Connection::new_from_address(&self.config.addresses[cursor]).await {
                Ok(conn) => {
                    self.conns.lock().unwrap().borrow_mut().push_back(conn);
                    count += 1;
                }
                Err(_) => (),
            };
            loop_count += 1;
            if loop_count > loop_limit {
                // Can't get so many connections, avoid dead loop
                break;
            }
        }
        // Release ordering make sure inc happened after creating new connections
        self.conns_count
            .borrow_mut()
            .fetch_add(count as usize, std::sync::atomic::Ordering::Release);
    }

    // cursor on the server addresses
    fn cursor(&self) -> usize {
        if self
            .cursor
            .borrow()
            .load(std::sync::atomic::Ordering::Relaxed)
            >= self.config.addresses.len()
        {
            self.cursor
                .borrow_mut()
                .store(0, std::sync::atomic::Ordering::Relaxed);
            0
        } else {
            self.cursor
                .borrow_mut()
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        }
    }
}
