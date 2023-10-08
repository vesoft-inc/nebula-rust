pub mod data;
pub mod datadeal;


use nebula_fbthrift_graph_v3::ExecutionResponse;
use nebula_fbthrift_storage_v3::ScanResponse;
use serde::Deserialize;

use crate::v3::de::data::{DataDeserializeError, DataDeserializer};
use crate::v3::de::datadeal::{process_value,process_column_name,scan_vertex_result,scan_edge_result,ProcessError};



pub fn deserialize_scan_response(
    scan_response: & ScanResponse, is_vertex: bool
) -> Result<Vec<String>, ProcessError> {
    let mut data_set: Vec<String> = vec![];

   
    let (names, rows) = match &scan_response.props {
        Some(set) => {
            if set.column_names.is_empty() {
                return Ok(data_set); 
            }
            (&set.column_names, &set.rows)
        }
        _ => return Ok(data_set), 
    };

    // 遍历每一个属性
    let mut processed_column_names = Vec::new(); 
    for col_name in names.iter() {
        // 创建一个 DataDeserializer 实例用于解析数据

        let de_col_name = process_column_name(col_name)?;

        processed_column_names.push(de_col_name.clone());
        
    }

    // 遍历每一行数据
    for row in rows.iter() {

        let mut processed_values = Vec::new(); 
        for value in row.values.iter() {
        // 处理 Value
            let value_result = process_value(value)?;

            processed_values.push(value_result.clone());
           
        }

        if is_vertex {
            // 如果is_vertex为真，执行scan_vertex_result函数
            let vertex_result = scan_vertex_result(&processed_column_names, &processed_values)?;
            
            data_set.push(vertex_result);
            
        } else {
            // 否则，执行scan_edge_result函数
            let edge_result = scan_edge_result(&processed_column_names, &processed_values)?;

            data_set.push(edge_result);
        }   
        
    }



    // 返回结果，包含反序列化后的数据集合
    Ok(data_set)
}



pub fn deserialize_scan_struct_response<'de, D: Deserialize<'de>>(
    scan_response: &'de ScanResponse,
) -> Result<Vec<D>, DataDeserializeError> {
    let mut data_set: Vec<D> = vec![];

    let (names, rows) = match &scan_response.props {
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


