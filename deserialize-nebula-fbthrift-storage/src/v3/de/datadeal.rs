use nebula_fbthrift_graph_v3::dependencies::common::types::Value;



pub fn scan_vertex_result(col_names:&Vec<String>,rows:&Vec<String>) ->Result<String, ProcessError> {
    
    if col_names.len() != rows.len() {
        return Err(ProcessError(DataDealError::VertexResultError));
    }
    
    let mut vertex_props = Vec::new(); 
    // 使用迭代器跳过前一个元素（下标为0）
    let mut first_part = String::new();
    for col_name in col_names.iter().skip(2) {
        // 使用split方法将字符串按照`.`分割成多个部分，并将结果转换为迭代器
        let parts: Vec<&str> = col_name.split('.').collect();
        first_part = parts[0].to_string().clone();
        // 如果分割后有多个部分，取第二个部分（下标为1），否则使用整个字符串
        let second_part = if parts.len() > 1 {
            parts[1]
        } else {
            col_name
        };
        
        vertex_props.push(second_part.to_string().clone());
        
        
    }
    
    let mut result = String::from("(");
    
    result.push_str(&format!("{}:{}{{", rows.get(1).map(|s| s.as_str()).unwrap_or(""),first_part));

    // 遍历 col_names 和 values，构建每对键值对
    for (i, prop) in vertex_props.iter().enumerate() {
        //let value = rows.get(i+1).unwrap_or(&String::new()); // 获取对应的值，如果越界则使用空字符串
        let value = rows.get(i + 2).map(|s| s.as_str()).unwrap_or("");
        result.push_str(&format!("{}: {}", prop, value));

        if i < vertex_props.len() - 1 {
            result.push_str(", "); // 添加逗号分隔
        }
    }

    result.push_str("})");

    println!("{}", result);


    Ok(result)
}

pub fn scan_edge_result(col_names:&Vec<String>,rows:&Vec<String>) ->Result<String, ProcessError> {
    
    if col_names.len() != rows.len() {
        return Err(ProcessError(DataDealError::EdgeResultError));
    }
    
    let mut edge_props = Vec::new(); 
    // 使用迭代器跳过前一个元素（下标为0）
    let mut first_part = String::new();
    for col_name in col_names.iter().skip(4) {
        // 使用split方法将字符串按照`.`分割成多个部分，并将结果转换为迭代器
        let parts: Vec<&str> = col_name.split('.').collect();
        first_part = parts[0].to_string().clone();
        // 如果分割后有多个部分，取第二个部分（下标为1），否则使用整个字符串
        let second_part = if parts.len() > 1 {
            parts[1]
        } else {
            col_name
        };
        edge_props.push(second_part.to_string().clone());
   
        
    }
    
    
    
    let mut result = String::from(&format!("({})-[:{}@{}{{", rows.get(0).map(|s| s.as_str()).unwrap_or(""),first_part,rows.get(2).map(|s| s.as_str()).unwrap_or("")));


    // 遍历 col_names 和 values，构建每对键值对
    for (i, prop) in edge_props.iter().enumerate() {
        //let value = rows.get(i+1).unwrap_or(&String::new()); // 获取对应的值，如果越界则使用空字符串
        let value = rows.get(i + 4).map(|s| s.as_str()).unwrap_or("");
        result.push_str(&format!("{}: {}", prop, value));

        if i < edge_props.len() - 1 {
            result.push_str(", "); // 添加逗号分隔
        }
    }

    result.push_str(&format!("}}]->({})", rows.get(3).map(|s| s.as_str()).unwrap_or("")));


    println!("{}", result);


    Ok(result)
}

// 定义 process_column_name 函数来处理字节数据
pub fn process_column_name(bytes: &[u8]) -> Result<String, ProcessError> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(decoded_string) => Ok(decoded_string),
        Err(_) => Err(ProcessError(DataDealError::ColumnNameError)),
    }
}


// 定义 process_value 函数来处理 Value
pub fn process_value(value: &Value) -> Result<String, ProcessError> {
    match value {
        Value::bVal(bool_value) => {
            Ok(format!("{}", bool_value))
        }
        Value::iVal(integer_value) => {
            Ok(format!("{}", integer_value))
        }
        Value::fVal(double_value) => {
            // 提取 Double 中的 f64 值
            let float_value = double_value.0;
            Ok(format!("{}", float_value))
        }
        Value::sVal(binary_data) => {
            let decoded_string = String::from_utf8(binary_data.to_vec()).map_err(|_| ProcessError(DataDealError::BinaryDecodeError))?;
            Ok(format!("{}", decoded_string))
        }
        Value::dVal(date_value) => {
            Ok(format!("{}-{:02}-{:02}", date_value.year, date_value.month, date_value.day))
        }
        Value::tVal(time_value) => {
            Ok(format!("{:02}:{:02}:{:02}.{:06}", time_value.hour, time_value.minute, time_value.sec, time_value.microsec))
        }
        Value::dtVal(dt_value) => {
            Ok(format!("{}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}", dt_value.year, dt_value.month, dt_value.day, dt_value.hour, dt_value.minute, dt_value.sec, dt_value.microsec))
        }
        Value::duVal(duration_value) => {
            Ok(format!("{} months, {} seconds, {} microseconds", duration_value.months, duration_value.seconds, duration_value.microseconds))
        }
        _ => Err(ProcessError(DataDealError::ValueError)),
    }
}

#[derive(Debug)]
pub enum DataDealError {
    VertexResultError,
    EdgeResultError,
    ColumnNameError,
    ValueError,
    BinaryDecodeError,
    Custom(String),
}


#[derive(Debug)]
pub struct ProcessError(DataDealError);

impl std::error::Error for ProcessError {}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            DataDealError::Custom(msg) => write!(f, "DataDeal Error: {}", msg),
            DataDealError::VertexResultError => write!(f, "Vertex Result Error"),
            DataDealError::EdgeResultError => write!(f, "Edge Result Error"),
            DataDealError::ColumnNameError => write!(f, "Column Name Error"),
            DataDealError::ValueError => write!(f, "Value Error"),
            DataDealError::BinaryDecodeError => write!(f, "Binary Decode Error"),
        }
    }
}
