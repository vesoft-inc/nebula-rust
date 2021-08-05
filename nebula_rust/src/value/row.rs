/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */

use common::types::Row;

pub trait RowValue {
    fn new(row: &[common::types::Value]) -> Self;

    fn len(&self) -> usize;
}

impl RowValue for Row {
    fn new(row: &[common::types::Value]) -> Self {
        Row {
            values: row.to_vec(),
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.values.len()
    }
}
