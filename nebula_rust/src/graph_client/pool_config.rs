/* Copyright (c) 2020 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */

#[derive(Debug, Default, Clone)]
pub struct PoolConfig {
    /// connection timeout in ms
    pub timeout: u32,
    pub idle_time: u32,
    /// max limit count of connections in pool
    pub max_connection_pool_size: u32,
    /// min limit count of connections in pool, also the initial count if works well
    pub min_connection_pool_size: u32,
    /// address of graph server
    pub addresses: std::vec::Vec<String>,
}

impl PoolConfig {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn timeout(&mut self, timeout: u32) -> &mut Self {
        self.timeout = timeout;
        self
    }

    #[inline]
    pub fn idle_time(&mut self, idle_time: u32) -> &mut Self {
        self.idle_time = idle_time;
        self
    }

    #[inline]
    pub fn max_connection_pool_size(&mut self, size: u32) -> &mut Self {
        self.max_connection_pool_size = size;
        self
    }

    #[inline]
    pub fn min_connection_pool_size(&mut self, size: u32) -> &mut Self {
        self.min_connection_pool_size = size;
        self
    }

    #[inline]
    pub fn addresses(&mut self, addresses: std::vec::Vec<String>) -> &mut Self {
        self.addresses = addresses;
        self
    }

    #[inline]
    pub fn address(&mut self, address: String) -> &mut Self {
        self.addresses.push(address);
        self
    }
}
