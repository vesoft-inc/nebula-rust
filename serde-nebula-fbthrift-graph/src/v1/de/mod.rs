pub mod data;

use nebula_fbthrift_graph_v1::ExecutionResponse;
use serde::Deserialize;

use crate::v1::de::data::{DataDeserializeError, DataDeserializer};

pub fn deserialize_execution_response<'de, D: Deserialize<'de>>(
    execution_response: &'de ExecutionResponse,
) -> Result<Vec<D>, DataDeserializeError> {
    let mut data_set: Vec<D> = vec![];

    let names = match &execution_response.column_names {
        Some(column_names) if !column_names.is_empty() => column_names,
        _ => return Ok(data_set),
    };

    let rows = match &execution_response.rows {
        Some(rows) => rows,
        None => return Ok(data_set),
    };

    for row in rows.iter() {
        let mut data_deserializer = DataDeserializer::new(names, &row.columns);

        let data = D::deserialize(&mut data_deserializer)?;

        data_set.push(data);
    }

    Ok(data_set)
}
