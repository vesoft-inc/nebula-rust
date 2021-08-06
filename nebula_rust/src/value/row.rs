/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */

pub trait Row {
    /// Construct row by columns name
    fn new(row: &[common::types::Value]) -> Self;

    /// Construct row by vec of column name
    fn from_vec(row: std::vec::Vec<common::types::Value>) -> Self;

    /// Get row length
    fn len(&self) -> usize;
}

impl Row for common::types::Row {
    #[inline]
    fn new(row: &[common::types::Value]) -> Self {
        common::types::Row {
            values: row.to_vec(),
        }
    }

    #[inline]
    fn from_vec(row: std::vec::Vec<common::types::Value>) -> Self {
        common::types::Row { values: row }
    }

    #[inline]
    fn len(&self) -> usize {
        self.values.len()
    }
}
