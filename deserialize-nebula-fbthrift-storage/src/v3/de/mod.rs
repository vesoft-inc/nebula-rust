// 在 Rust 中，mod 关键字用于创建模块，而 pub 关键字用于声明模块或模块中的项对外可见。
// 因此，如果在 serde-nebula/src/v3/de/mod.rs 文件中写了 pub mod data;，那么 data 模块就会被导出为 v3::de 模块的一个公共子模块。
pub mod data;
pub mod datadeal;

// 导入必要的库和模块
use nebula_fbthrift_graph_v3::ExecutionResponse;
use nebula_fbthrift_storage_v3::ScanResponse;
use serde::Deserialize;

use crate::v3::de::data::{DataDeserializeError, DataDeserializer};
use crate::v3::de::datadeal::{process_value,process_column_name,scan_vertex_result,scan_edge_result,ProcessError};



pub fn deserialize_scan_response(
    scan_response: & ScanResponse, is_vertex: bool
) -> Result<Vec<String>, ProcessError> {
    let mut data_set: Vec<String> = vec![];

    // 判断是否存在数据
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








// 下面是老版的
pub fn deserialize_execution_response<'de, D: Deserialize<'de>>(
    execution_response: &'de ExecutionResponse,
) -> Result<Vec<D>, DataDeserializeError> {
    // 创建一个空的数据集合 data_set 每一行都是结构体D
    let mut data_set: Vec<D> = vec![];

    // 判断是否存在数据
    let (names, rows) = match &execution_response.data {
        Some(set) => {
            if set.column_names.is_empty() {
                return Ok(data_set); // 如果数据为空，则直接返回空数据集合
            }
            (&set.column_names, &set.rows)
        }
        _ => return Ok(data_set), // 如果数据不存在，则直接返回空数据集合
    };

    // 遍历每一行数据
    for col_name in names.iter() {
 

        match process_column_name(col_name) {
            Ok(col_name) => {
                println!("Column Name: {}", col_name);
            }
            Err(err) => {
                eprintln!("Error processing column name: {}", err);
            }
        }
        

    }

    // 遍历每一行数据
    for row in rows.iter() {
       

        for value in row.values.iter() {
        // 处理 Value
            match process_value(value) {
                Ok(result) => {
                    println!("Value: {}", result);
                }
                Err(err) => {
                    eprintln!("Error processing value: {}", err);
                }
            }
        }

        // Vec<Value, Global> 的引用转换为对 Value 类型的切片的引用
        let mut data_deserializer = DataDeserializer::new(names, &row.values);

        // 使用 Deserialize trait 反序列化数据
        let data = D::deserialize(&mut data_deserializer)?;

        
        // 将反序列化的数据添加到 data_set 中
        data_set.push(data);
    }

    // 返回结果，包含反序列化后的数据集合
    Ok(data_set)
}

