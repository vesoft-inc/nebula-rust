/*
core: 这是 Rust 的核心库，包含了所有基本的 Rust 类型和功能。它不依赖于标准库，因此可以在嵌入式系统等环境中使用。
iter::Peekable: iter 是 core 模块下的子模块，它包含了与迭代器相关的类型和特性。Peekable 是 iter 模块中的一个类型，它表示一个能够 "窥视"（peek）下一个元素的迭代器。这意味着你可以查看迭代器中下一个元素的值，而不必移动迭代器的位置。
ops::Div as _: ops 是 core 模块下的子模块，包含了运算符重载相关的特性和类型。在这里，Div 是 ops 模块中的一个 trait，表示除法运算符。as _ 的部分将 Div 重命名为下划线 _，这意味着在这个作用域中，我们不会直接使用 Div 这个名称，而是用 _ 来代替它。这通常是因为我们不打算直接使用 Div，而只是希望将它引入到当前作用域中以使用其特性。
slice::Iter: slice 是 core 模块下的子模块，包含了与切片（slice）相关的类型和功能。Iter 是 slice 模块中的一个类型，表示切片的迭代器。它用于在切片上进行迭代操作。
*/
use core::{iter::Peekable, ops::Div as _, slice::Iter};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use nebula_fbthrift_graph::v3::dependencies::common::types::Value;
use serde::de::{
    self,
    value::{BorrowedBytesDeserializer, SeqDeserializer},
    DeserializeSeed, Deserializer, MapAccess, Visitor,
};

pub struct DataDeserializer<'a> {
    names_iter: Iter<'a, Vec<u8>>,
    values_iter: Peekable<Iter<'a, Value>>,
    field: usize,  // 定义字段 field，类型为 usize，用于追踪当前字段的索引
}

impl<'a> DataDeserializer<'a> {
    pub fn new(names: &'a [Vec<u8>], values: &'a [Value]) -> Self {
        let names_iter = names.iter();
        let values_iter = values.iter().peekable();

        Self {
            names_iter,
            values_iter,
            field: 0,
        }
    }

    fn next_name(&mut self) -> Option<&'a Vec<u8>> { // next_name: 这个方法用于获取下一个字段的名称。
        self.names_iter.next()
    }

    fn next_value(&mut self) -> Result<&'a Value, DataDeserializeError> { // next_value: 这个方法用于获取下一个字段的值。
        match self.values_iter.next() {
            Some(row) => {
                self.field += 1;
                Ok(row)
            }
            None => Err(DataDeserializeError::new(
                None,
                DataDeserializeErrorKind::UnexpectedEndOf,
            )),
        }
    }

    fn peek_value(&mut self) -> Option<&&'a Value> { //peek_value: 这个方法用于预览下一个字段的值，但不移动迭代器的位置。
        self.values_iter.peek()
    }

    // 函数使用 DataDeserializeError::new 来创建错误实例，传递了位置信息和错误类型，并将该错误实例返回。
    fn error(&self, kind: DataDeserializeErrorKind) -> DataDeserializeError {
        DataDeserializeError::new(Some(self.field.saturating_sub(1)), kind)
    }
}

// 为DataDeserializer<'de>设定一个反序列化器
impl<'a, 'de> Deserializer<'de> for &'a mut DataDeserializer<'de> {
    type Error = DataDeserializeError; //自定义error类型

    // deserialize_any 方法会始终导致一个错误，表示这个方法的功能尚未实现，需要在后续的代码中进行具体实现。
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
    // where V: Visitor<'de>：这是一个泛型约束，它表明泛型参数 V 必须是一个实现了 'de 生命周期的 Visitor trait 的类型。'de 生命周期通常表示数据反序列化的生命周期。
        V: Visitor<'de>,
    {
        match self.next_value()? {
    // 将解析到的布尔值 v 传递给了 visitor，然后返回 visit_bool 方法的结果。这个分支表示成功反序列化布尔值，并将其传递给 visitor 处理。            
            Value::bVal(v) => visitor.visit_bool(*v),
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_bool(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::iVal(v) => match i8::try_from(*v) {
                Ok(v) => visitor.visit_i8(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_i8(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::iVal(v) => match i16::try_from(*v) {
                Ok(v) => visitor.visit_i16(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_i16(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::iVal(v) => match i32::try_from(*v) {
                Ok(v) => visitor.visit_i32(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_i32(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::iVal(v) => visitor.visit_i64(*v),
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_i64(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::iVal(v) => match u8::try_from(*v) {
                Ok(v) => visitor.visit_u8(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            Value::sVal(v) => visitor.visit_u8(v[0]),
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_u8(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::iVal(v) => match u16::try_from(*v) {
                Ok(v) => visitor.visit_u16(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_u16(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::iVal(v) => match u32::try_from(*v) {
                Ok(v) => visitor.visit_u32(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_u32(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::iVal(v) => match u64::try_from(*v) {
                Ok(v) => visitor.visit_u64(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_u64(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::fVal(v) => visitor.visit_f32(v.0 as f32),
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_f32(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::fVal(v) => visitor.visit_f64(v.0),
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_f64(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::sVal(v) => visitor.visit_string(String::from_utf8_lossy(v).to_string()),
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                visitor.visit_string(Default::default())
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.peek_value() {
            Some(_) => visitor.visit_some(self),
            None => visitor.visit_none(),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::sVal(v) => {
                let mut seq_deserializer = SeqDeserializer::new(v.iter().copied());
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                let v: Vec<u8> = Default::default();
                let mut seq_deserializer = SeqDeserializer::new(v.iter().copied());
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::dVal(v) => {
                let mut seq_deserializer =
                    SeqDeserializer::new(vec![v.year, v.month as i16, v.day as i16].into_iter());
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            Value::tVal(v) => {
                let mut seq_deserializer = SeqDeserializer::new(
                    vec![
                        v.hour as i16,
                        v.minute as i16,
                        v.sec as i16,
                        v.microsec.div(1000) as i16,
                    ]
                    .into_iter(),
                );
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            Value::dtVal(v) => {
                let mut seq_deserializer = SeqDeserializer::new(
                    vec![
                        v.year,
                        v.month as i16,
                        v.day as i16,
                        v.hour as i16,
                        v.minute as i16,
                        v.sec as i16,
                        v.microsec.div(1000) as i16,
                        0i16,
                    ]
                    .into_iter(),
                );
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                // TODO,
                let v: Vec<u8> = Default::default();
                let mut seq_deserializer = SeqDeserializer::new(v.iter().copied());
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            Value::dVal(v) => {
                let mut seq_deserializer =
                    SeqDeserializer::new(vec![v.year, v.month as i16, v.day as i16].into_iter());
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            Value::tVal(v) => {
                let mut seq_deserializer = SeqDeserializer::new(
                    vec![
                        v.hour as i16,
                        v.minute as i16,
                        v.sec as i16,
                        v.microsec.div(1000) as i16,
                    ]
                    .into_iter(),
                );
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            Value::dtVal(v) => {
                let mut seq_deserializer = SeqDeserializer::new(
                    vec![
                        v.year,
                        v.month as i16,
                        v.day as i16,
                        v.hour as i16,
                        v.minute as i16,
                        v.sec as i16,
                        v.microsec.div(1000) as i16,
                        0i16,
                    ]
                    .into_iter(),
                );
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            Value::UnknownField(v) => {
                assert_eq!(v, &-1);
                // TODO,
                let v: Vec<u8> = Default::default();
                let mut seq_deserializer = SeqDeserializer::new(v.iter().copied());
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // ref https://github.com/BurntSushi/rust-csv/blob/1.1.3/src/deserializer.rs#L554-L563
        let _ = self.next_value()?;
        visitor.visit_unit()
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

impl<'a, 'de> MapAccess<'de> for &'a mut DataDeserializer<'de> {
    type Error = DataDeserializeError;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let name = match self.next_name() {
            Some(name) => name,
            None => return Ok(None),
        };
        // 这是方法的返回类型，它表示反序列化结果。它是一个 Result，可能包含了键的值（K::Value），或者返回 Ok(None) 表示没有更多的键可用，或者包含了错误（Self::Error）表示反序列化过程中出现了问题。
        seed.deserialize(BorrowedBytesDeserializer::new(name))
            .map(Some)
    }

    fn next_value_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<K::Value, Self::Error> {
        seed.deserialize(&mut **self)
    }
}

//
//
// 定义了一个结构体 DataDeserializeError 用于表示数据反序列化时可能出现的错误
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataDeserializeError {
    pub field: Option<usize>, // 记录错误所在字段的位置，可选值
    pub kind: DataDeserializeErrorKind, // 记录错误的种类
}

impl DataDeserializeError {
    // DataDeserializeError 结构体的构造函数，用于创建一个新的错误实例
    pub fn new(field: Option<usize>, kind: DataDeserializeErrorKind) -> Self {
        Self { field, kind }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataDeserializeErrorKind { // 定义了一个枚举 DataDeserializeErrorKind，表示可能的错误种类
    UnexpectedEndOf, // 意外的数据结束
    TypeMismatch, // 数据类型不匹配
    Unimplemented, // 未实现的错误
    Custom(String), // 自定义错误，包含一条错误消息
}

impl DataDeserializeErrorKind {
    // 获取错误种类的描述信息
    #[allow(deprecated)] // 用于禁用对被标记为"deprecated"的功能的警告。
    fn description(&self) -> &str {
        //它的作用是将 DataDeserializeErrorKind 枚举中的所有成员引入当前作用域，以便在后续的代码中可以直接使用这些成员，而不需要每次都使用完整的路径。
        use self::DataDeserializeErrorKind::*; 
        
        match *self {
            UnexpectedEndOf => "Unexpected end of",
            TypeMismatch => "Type mismatch",
            Unimplemented => "Unimplemented",
            Custom(ref msg) => msg, // 如果是自定义错误，返回自定义消息
        }
    }
}


impl std::error::Error for DataDeserializeError { // 实现 std::error::Error trait，允许 DataDeserializeError 作为标准错误处理
    // 自定义的错误都要实现error错误 实现方法fn description(&self) -> &str
    fn description(&self) -> &str {
        self.kind.description() // 委托给错误种类的描述信息
    }
}

impl de::Error for DataDeserializeError { // 实现 serde::de::Error trait，允许 DataDeserializeError 作为 Serde 反序列化错误处理
    fn custom<T: core::fmt::Display>(msg: T) -> DataDeserializeError {
        DataDeserializeError {
            field: None, // 字段位置为空
            kind: DataDeserializeErrorKind::Custom(msg.to_string()), // 错误种类为自定义错误
        }
    }
}

impl core::fmt::Display for DataDeserializeError { // 实现 core::fmt::Display trait，用于将错误格式化为字符串
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(field) = self.field {
            write!(f, "field {field}: {}", self.kind) // 如果有字段位置信息，显示字段位置
        } else {
            write!(f, "{}", self.kind) // 否则只显示错误种类
        }
    }
}

impl core::fmt::Display for DataDeserializeErrorKind { // 实现 core::fmt::Display trait，用于将错误种类格式化为字符串
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::DataDeserializeErrorKind::*;

        match *self {
            UnexpectedEndOf => write!(f, "{}", self.description()),
            TypeMismatch => write!(f, "{}", self.description()),
            Unimplemented => write!(f, "{}", self.description()),
            Custom(ref msg) => write!(f, "{msg}"),
        }
    }
}

// 在 Rust 中，标准库，From trait 允许你定义类型之间的转换。这通常用于将一种类型转换为另一种类型，以便在不同的上下文中使用。
impl From<DataDeserializeError> for IoError { // 提供了一种将 DataDeserializeError 转换为标准库的 IoError 错误的方式。
    fn from(err: DataDeserializeError) -> IoError {
        IoError::new(IoErrorKind::InvalidInput, err)
    }
}

#[cfg(test)]
mod tests {
    // 接下来，在 serde-nebula/src/v3/de/tests 模块中的测试模块中使用 super::* 来引入父模块的所有公共项。因此，mod tests 的父模块是 v3::de，它包含了 data 模块以及其他可能的子模块或项。
    use super::*; // 要用本文件的东西 DataDeserializer::new

    use float_cmp::approx_eq;
    use nebula_fbthrift_graph_v3::dependencies::common::{double::Double, types};
    use serde::{de::DeserializeOwned, Deserialize};
    // erde_repr 是一个用于 Serde 的宏扩展库，它的主要功能是为枚举类型提供一种更紧凑的表示方式，以便进行序列化和反序列化。
    use serde_repr::Deserialize_repr;

    use crate::datetime::{self, Date, Day, Hour, Millisec, Minute, Month, Second, Time, Year};

    fn de<D: DeserializeOwned>(
        names: Vec<&str>, // 用于标识将在 values 中出现的数据的字段
        values: Vec<Value>,
    ) -> Result<D, Box<dyn std::error::Error>> {
        //从字符串切片转换为 Vec<Vec<u8>> 类型。首先，通过 into_iter() 将字符串切片 names 转换为迭代器，然后使用 map 函数将每个字符串转换为字节数组 (Vec<u8>)，
        //最后使用 collect 将结果收集到 names 变量中。这是因为 DataDeserializer 需要字段名作为字节数组。
        let names: Vec<_> = names.into_iter().map(|x| x.as_bytes().to_vec()).collect();

        let mut data_deserializer = DataDeserializer::new(&names, &values);

        // 从原始错误类型转换为 Box<dyn std::error::Error>，这是 Rust 中一种通用的错误处理方式。这个转换操作是通过 Into trait 自动实现的，它允许类型转换为目标类型。
        D::deserialize(&mut data_deserializer).map_err(Into::into)
    }

    #[test]
    fn with_b_val() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: bool,
            b: bool,
        }
        

        let v: Foo = de(vec!["a", "b"], vec![Value::bVal(true), Value::bVal(false)])?;

        assert!(v.a);
        assert!(!v.b);

        Ok(())
    }

    #[test]
    fn with_i_val() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize_repr, PartialEq, Debug)]
        //使用了 repr(u8) 属性来指定底层数据类型是 u8。这意味着在序列化和反序列化时，枚举的成员将被表示为整数，而不是字符串。
        #[repr(u8)] 
        enum State {
            Pending = 1,
            Done = 2,
        }

        #[derive(Deserialize)]
        struct Foo {
            a: i64,
            b: i32,
            c: i16,
            d: i8,
            e: u64,
            f: u32,
            g: u16,
            h: u8,
            state: State,
        }

        let v: Foo = de(
            vec!["a", "b", "c", "d", "e", "f", "g", "h", "state"],
            vec![
                Value::iVal(1),
                Value::iVal(2),
                Value::iVal(3),
                Value::iVal(4),
                Value::iVal(5),
                Value::iVal(6),
                Value::iVal(7),
                Value::iVal(8),
                Value::iVal(2),
            ],
        )?;

        assert_eq!(v.a, 1);
        assert_eq!(v.b, 2);
        assert_eq!(v.c, 3);
        assert_eq!(v.d, 4);
        assert_eq!(v.e, 5);
        assert_eq!(v.f, 6);
        assert_eq!(v.g, 7);
        assert_eq!(v.h, 8);
        assert_eq!(v.state, State::Done);

        Ok(())
    }

    #[test]
    fn with_f_val() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: f64,
        }

        let v: Foo = de(vec!["a"], vec![Value::fVal(Double(1_f64))])?;

        assert!(approx_eq!(f64, v.a, 1_f64));

        Ok(())
    }

    #[test]
    fn with_s_val() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: String,
            b: Vec<u8>,
        }

        let v: Foo = de(
            vec!["a", "b"],
            vec![
                Value::sVal(b"String".to_vec()),
                Value::sVal(b"Vec<u8>".to_vec()),
            ],
        )?;

        assert_eq!(v.a, "String");
        assert_eq!(v.b, b"Vec<u8>");

        Ok(())
    }

    #[test]
    fn with_d_val() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: (Year, Month, Day),
            b: Date,
        }

        let v: Foo = de(
            vec!["a", "b"],
            vec![
                Value::dVal(types::Date {
                    year: 2020,
                    month: 1,
                    day: 2,
                    ..Default::default()
                }),
                Value::dVal(types::Date {
                    year: 2020,
                    month: 1,
                    day: 3,
                    ..Default::default()
                }),
            ],
        )?;

        assert_eq!(v.a, (2020, 1, 2));
        assert_eq!(v.b, Date(2020, 1, 3));

        Ok(())
    }

    #[test]
    fn with_t_val() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: (Hour, Minute, Second, Millisec),
            b: Time,
        }

        let v: Foo = de(
            vec!["a", "b"],
            vec![
                Value::tVal(types::Time {
                    hour: 1,
                    minute: 2,
                    sec: 3,
                    microsec: 8001,
                    ..Default::default()
                }),
                Value::tVal(types::Time {
                    hour: 4,
                    minute: 5,
                    sec: 6,
                    microsec: 9001,
                    ..Default::default()
                }),
            ],
        )?;

        assert_eq!(v.a, (1, 2, 3, 8));
        assert_eq!(v.b, Time(4, 5, 6, 9));

        Ok(())
    }

    #[test]
    fn with_dt_val() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: datetime::DateTime,
        }

        let v: Foo = de(
            vec!["a"],
            vec![Value::dtVal(types::DateTime {
                year: 2020,
                month: 1,
                day: 2,
                hour: 3,
                minute: 4,
                sec: 5,
                microsec: 9001,
                ..Default::default()
            })],
        )?;

        assert_eq!(v.a, datetime::DateTime(2020, 1, 2, 3, 4, 5, 9, 0));

        Ok(())
    }

    #[test]
    fn with_unknown_field() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: i32,
        }

        let v: Foo = de(vec!["a"], vec![Value::UnknownField(-1)])?;

        assert_eq!(v.a, 0);

        Ok(())
    }

    #[test]
    fn with_multiple() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: bool,
            b: i64,
            c: String,
        }

        let v: Foo = de(
            vec!["a", "b", "c"],
            vec![
                Value::bVal(true),
                Value::iVal(1),
                Value::sVal(b"3".to_vec()),
            ],
        )?;

        assert!(v.a);
        assert_eq!(v.b, 1);
        assert_eq!(v.c, "3");

        Ok(())
    }

    #[test]
    fn with_unit() -> Result<(), Box<dyn std::error::Error>> {
        de::<()>(vec!["a"], vec![Value::bVal(true)])?;

        Ok(())
    }

    #[test]
    fn with_option() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: Option<bool>,
        }

        let v: Foo = de(vec!["a"], vec![Value::bVal(true)])?;

        assert_eq!(v.a, Some(true));

        Ok(())
    }
}
