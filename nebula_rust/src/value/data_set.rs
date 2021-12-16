/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */

pub trait DataSet {
    /// Construct data set with name of columns
    fn new(col_names: &[String]) -> Self;

    /// Construct data set from vec of columns name
    fn from_columns_name(col_names: std::vec::Vec<String>) -> Self;

    /// push one row into back of data set
    fn push(&mut self, row: common::types::Row);

    /// Get rows size
    fn len(&self) -> usize;

    /// Get count of columns
    fn cols_len(&self) -> usize;
}

impl DataSet for common::types::DataSet {
    fn new(col_names: &[String]) -> Self {
        let cols_bytes = col_names.into_iter().map(|s| s.as_bytes().to_vec()).collect();
        common::types::DataSet {
            column_names: cols_bytes,
            rows: vec![],
        }
    }

    fn from_columns_name(col_names: std::vec::Vec<String>) -> Self {
        let cols_bytes = col_names
            .into_iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();
        common::types::DataSet {
            column_names: cols_bytes,
            rows: vec![],
        }
    }

    #[inline]
    fn push(&mut self, row: common::types::Row) {
        self.rows.push(row);
    }

    #[inline]
    fn len(&self) -> usize {
        self.rows.len()
    }

    #[inline]
    fn cols_len(&self) -> usize {
        self.column_names.len()
    }
}
