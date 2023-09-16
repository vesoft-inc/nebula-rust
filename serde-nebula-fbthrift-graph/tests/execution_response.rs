use nebula_fbthrift_graph_v1::{
    types::{ColumnValue, ErrorCode, RowValue},
    ExecutionResponse,
};
use serde::Deserialize;

use serde_nebula_fbthrift_graph::v1::de::deserialize_execution_response;

#[derive(Deserialize, PartialEq, Debug)]
struct Foo {
    integer: i64,
    str: String,
}

#[test]
fn with_none_column_names() -> Result<(), Box<dyn std::error::Error>> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: None,
        rows: None,
        space_name: None,
        warning_msg: None,
        ..Default::default()
    };

    let foo_set = deserialize_execution_response::<Foo>(&execution_response)?;
    assert_eq!(foo_set.len(), 0);

    Ok(())
}

#[test]
fn with_empty_column_names() -> Result<(), Box<dyn std::error::Error>> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: Some(vec![]),
        rows: None,
        space_name: None,
        warning_msg: None,
        ..Default::default()
    };

    let foo_set = deserialize_execution_response::<Foo>(&execution_response)?;
    assert_eq!(foo_set.len(), 0);

    Ok(())
}

#[test]
fn with_none_rows() -> Result<(), Box<dyn std::error::Error>> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: Some(vec![b"integer".to_vec(), b"str".to_vec()]),
        rows: None,
        space_name: None,
        warning_msg: None,
        ..Default::default()
    };

    let foo_set = deserialize_execution_response::<Foo>(&execution_response)?;
    assert_eq!(foo_set.len(), 0);

    Ok(())
}

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: Some(vec![b"integer".to_vec(), b"str".to_vec()]),
        rows: Some(vec![
            RowValue {
                columns: vec![ColumnValue::integer(1), ColumnValue::str(b"1".to_vec())],
                ..Default::default()
            },
            RowValue {
                columns: vec![ColumnValue::integer(2), ColumnValue::str(b"2".to_vec())],
                ..Default::default()
            },
        ]),
        space_name: None,
        warning_msg: None,
        ..Default::default()
    };

    let foo_set = deserialize_execution_response::<Foo>(&execution_response)?;
    assert_eq!(foo_set.len(), 2);
    let foo_first = foo_set.first().unwrap();
    assert_eq!(foo_first.integer, 1);
    assert_eq!(foo_first.str, "1");
    let foo_last = foo_set.last().unwrap();
    assert_eq!(foo_last.integer, 2);
    assert_eq!(foo_last.str, "2");

    Ok(())
}

#[test]
fn with_unit() -> Result<(), Box<dyn std::error::Error>> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: Some(vec![b"integer".to_vec(), b"str".to_vec()]),
        rows: Some(vec![
            RowValue {
                columns: vec![ColumnValue::integer(1), ColumnValue::str(b"1".to_vec())],
                ..Default::default()
            },
            RowValue {
                columns: vec![ColumnValue::integer(2), ColumnValue::str(b"2".to_vec())],
                ..Default::default()
            },
        ]),
        space_name: None,
        warning_msg: None,
        ..Default::default()
    };

    let foo_set = deserialize_execution_response::<()>(&execution_response)?;
    assert_eq!(foo_set.len(), 2);

    Ok(())
}
