pub mod data;

use nebula_fbthrift_graph_v2::ExecutionResponse;
use serde::Deserialize;

use crate::v2::de::data::{DataDeserializeError, DataDeserializer};

pub fn deserialize_execution_response<'de, D: Deserialize<'de>>(
    execution_response: &'de ExecutionResponse,
) -> Result<Vec<D>, DataDeserializeError> {
    let mut data_set: Vec<D> = vec![];

    let (names, rows) = match &execution_response.data {
        Some(set) => {
            if set.column_names.is_empty() {
                return Ok(data_set);
            }
            (&set.column_names, &set.rows)
        }
        _ => return Ok(data_set),
    };

    for row in rows.iter() {
        let mut data_deserializer = DataDeserializer::new(names, &row.values);

        let data = D::deserialize(&mut data_deserializer)?;

        data_set.push(data);
    }

    Ok(data_set)
}
