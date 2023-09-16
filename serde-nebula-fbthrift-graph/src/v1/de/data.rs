use core::{iter::Peekable, slice::Iter};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use nebula_fbthrift_graph_v1::types::ColumnValue;
use serde::de::{
    self,
    value::{BorrowedBytesDeserializer, SeqDeserializer},
    DeserializeSeed, Deserializer, MapAccess, Visitor,
};

pub struct DataDeserializer<'a> {
    names_iter: Iter<'a, Vec<u8>>,
    values_iter: Peekable<Iter<'a, ColumnValue>>,
    field: usize,
}

impl<'a> DataDeserializer<'a> {
    pub fn new(names: &'a [Vec<u8>], values: &'a [ColumnValue]) -> Self {
        let names_iter = names.iter();
        let values_iter = values.iter().peekable();

        Self {
            names_iter,
            values_iter,
            field: 0,
        }
    }

    fn next_name(&mut self) -> Option<&'a Vec<u8>> {
        self.names_iter.next()
    }

    fn next_value(&mut self) -> Result<&'a ColumnValue, DataDeserializeError> {
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

    fn peek_value(&mut self) -> Option<&&'a ColumnValue> {
        self.values_iter.peek()
    }

    fn error(&self, kind: DataDeserializeErrorKind) -> DataDeserializeError {
        DataDeserializeError::new(Some(self.field.saturating_sub(1)), kind)
    }
}

impl<'a, 'de> Deserializer<'de> for &'a mut DataDeserializer<'de> {
    type Error = DataDeserializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(self.error(DataDeserializeErrorKind::Unimplemented))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::bool_val(v) => visitor.visit_bool(*v),
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::integer(v) => match i8::try_from(*v) {
                Ok(v) => visitor.visit_i8(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::integer(v) => match i16::try_from(*v) {
                Ok(v) => visitor.visit_i16(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            ColumnValue::year(v) => visitor.visit_i16(*v),
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::integer(v) => match i32::try_from(*v) {
                Ok(v) => visitor.visit_i32(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::integer(v) | ColumnValue::id(v) | ColumnValue::timestamp(v) => {
                visitor.visit_i64(*v)
            }
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::integer(v) => match u8::try_from(*v) {
                Ok(v) => visitor.visit_u8(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            ColumnValue::str(v) => visitor.visit_u8(v[0]),
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::integer(v) => match u16::try_from(*v) {
                Ok(v) => visitor.visit_u16(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::integer(v) => match u32::try_from(*v) {
                Ok(v) => visitor.visit_u32(v),
                Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
            },
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::integer(v) | ColumnValue::id(v) | ColumnValue::timestamp(v) => {
                match u64::try_from(*v) {
                    Ok(v) => visitor.visit_u64(v),
                    Err(_) => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
                }
            }

            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::single_precision(v) => visitor.visit_f32(*v),
            _ => Err(self.error(DataDeserializeErrorKind::TypeMismatch)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.next_value()? {
            ColumnValue::double_precision(v) => visitor.visit_f64(*v),
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
            ColumnValue::str(v) => visitor.visit_string(String::from_utf8_lossy(v).to_string()),
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
            ColumnValue::str(v) => {
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
            ColumnValue::month(v) => {
                let mut seq_deserializer =
                    SeqDeserializer::new(vec![v.year, v.month as i16].into_iter());
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            ColumnValue::date(v) => {
                let mut seq_deserializer =
                    SeqDeserializer::new(vec![v.year, v.month as i16, v.day as i16].into_iter());
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
            ColumnValue::month(v) => {
                let mut seq_deserializer =
                    SeqDeserializer::new(vec![v.year, v.month as i16].into_iter());
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            ColumnValue::date(v) => {
                let mut seq_deserializer =
                    SeqDeserializer::new(vec![v.year, v.month as i16, v.day as i16].into_iter());
                let value = visitor.visit_seq(&mut seq_deserializer)?;
                seq_deserializer.end()?;
                Ok(value)
            }
            ColumnValue::datetime(v) => {
                let mut seq_deserializer = SeqDeserializer::new(
                    vec![
                        v.year,
                        v.month as i16,
                        v.day as i16,
                        v.hour as i16,
                        v.minute as i16,
                        v.second as i16,
                        v.millisec,
                        v.microsec,
                    ]
                    .into_iter(),
                );
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
//
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataDeserializeError {
    pub field: Option<usize>,
    pub kind: DataDeserializeErrorKind,
}
impl DataDeserializeError {
    pub fn new(field: Option<usize>, kind: DataDeserializeErrorKind) -> Self {
        Self { field, kind }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataDeserializeErrorKind {
    UnexpectedEndOf,
    TypeMismatch,
    Unimplemented,
    Custom(String),
}

impl DataDeserializeErrorKind {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        use self::DataDeserializeErrorKind::*;

        match *self {
            UnexpectedEndOf => "Unexpected end of",
            TypeMismatch => "Type mismatch",
            Unimplemented => "Unimplemented",
            Custom(ref msg) => msg,
        }
    }
}

impl std::error::Error for DataDeserializeError {
    fn description(&self) -> &str {
        self.kind.description()
    }
}

impl de::Error for DataDeserializeError {
    fn custom<T: core::fmt::Display>(msg: T) -> DataDeserializeError {
        DataDeserializeError {
            field: None,
            kind: DataDeserializeErrorKind::Custom(msg.to_string()),
        }
    }
}

impl core::fmt::Display for DataDeserializeError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(field) = self.field {
            write!(f, "field {field}: {}", self.kind)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

impl core::fmt::Display for DataDeserializeErrorKind {
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

impl From<DataDeserializeError> for IoError {
    fn from(err: DataDeserializeError) -> IoError {
        IoError::new(IoErrorKind::InvalidInput, err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{serde::ts_seconds, DateTime, TimeZone, Utc};
    use float_cmp::approx_eq;
    use nebula_fbthrift_graph_v1::types;
    use serde::{de::DeserializeOwned, Deserialize};
    use serde_repr::Deserialize_repr;

    use crate::datetime::{self, Date, Day, Month, Timestamp, Year, YearMonth};

    fn de<D: DeserializeOwned>(
        names: Vec<&str>,
        values: Vec<ColumnValue>,
    ) -> Result<D, Box<dyn std::error::Error>> {
        let names: Vec<_> = names.into_iter().map(|x| x.as_bytes().to_vec()).collect();

        let mut data_deserializer = DataDeserializer::new(&names, &values);

        D::deserialize(&mut data_deserializer).map_err(Into::into)
    }

    #[test]
    fn with_bool() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: bool,
            b: bool,
        }

        let v: Foo = de(
            vec!["a", "b"],
            vec![ColumnValue::bool_val(true), ColumnValue::bool_val(false)],
        )?;

        assert!(v.a);
        assert!(!v.b);

        Ok(())
    }

    #[test]
    fn with_integer() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize_repr, PartialEq, Debug)]
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
                ColumnValue::integer(1),
                ColumnValue::integer(2),
                ColumnValue::integer(3),
                ColumnValue::integer(4),
                ColumnValue::integer(5),
                ColumnValue::integer(6),
                ColumnValue::integer(7),
                ColumnValue::integer(8),
                ColumnValue::integer(2),
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
    fn with_id() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: i64,
            b: u64,
        }

        let v: Foo = de(vec!["a", "b"], vec![ColumnValue::id(1), ColumnValue::id(2)])?;

        assert_eq!(v.a, 1);
        assert_eq!(v.b, 2);

        Ok(())
    }

    #[test]
    fn with_single_precision() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: f32,
        }

        let v: Foo = de(vec!["a"], vec![ColumnValue::single_precision(1_f32)])?;

        assert!(approx_eq!(f32, v.a, 1_f32));

        Ok(())
    }

    #[test]
    fn with_double_precision() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: f64,
        }

        let v: Foo = de(vec!["a"], vec![ColumnValue::double_precision(1_f64)])?;

        assert!(approx_eq!(f64, v.a, 1_f64));

        Ok(())
    }

    #[test]
    fn with_str() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: String,
            b: Vec<u8>,
        }

        let v: Foo = de(
            vec!["a", "b"],
            vec![
                ColumnValue::str(b"String".to_vec()),
                ColumnValue::str(b"Vec<u8>".to_vec()),
            ],
        )?;

        assert_eq!(v.a, "String");
        assert_eq!(v.b, b"Vec<u8>");

        Ok(())
    }

    #[test]
    fn with_timestamp() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            #[serde(with = "ts_seconds")]
            a: DateTime<Utc>,
            b: Timestamp,
            c: i64,
            d: u64,
        }

        let v: Foo = de(
            vec!["a", "b", "c", "d"],
            vec![
                ColumnValue::timestamp(1577836800),
                ColumnValue::timestamp(1577836801),
                ColumnValue::timestamp(1577836802),
                ColumnValue::timestamp(1577836803),
            ],
        )?;

        assert_eq!(v.a, Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap());
        assert_eq!(v.b, Timestamp(1577836801));
        assert_eq!(v.c, 1577836802);
        assert_eq!(v.d, 1577836803);

        Ok(())
    }

    #[test]
    fn with_year() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            b: i16,
            a: Year,
        }

        let v: Foo = de(
            vec!["a", "b"],
            vec![ColumnValue::year(2020), ColumnValue::year(2021)],
        )?;

        assert_eq!(v.a, 2020);
        assert_eq!(v.b, 2021);

        Ok(())
    }

    #[test]
    fn with_month() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: (Year, Month),
            b: YearMonth,
        }

        let v: Foo = de(
            vec!["a", "b"],
            vec![
                ColumnValue::month(types::YearMonth {
                    year: 2020,
                    month: 1,
                    ..Default::default()
                }),
                ColumnValue::month(types::YearMonth {
                    year: 2020,
                    month: 2,
                    ..Default::default()
                }),
            ],
        )?;

        assert_eq!(v.a, (2020, 1));
        assert_eq!(v.b, YearMonth(2020, 2));

        Ok(())
    }

    #[test]
    fn with_date() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: (Year, Month, Day),
            b: Date,
        }

        let v: Foo = de(
            vec!["a", "b"],
            vec![
                ColumnValue::date(types::Date {
                    year: 2020,
                    month: 1,
                    day: 2,
                    ..Default::default()
                }),
                ColumnValue::date(types::Date {
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
    fn with_datetime() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: datetime::DateTime,
        }

        let v: Foo = de(
            vec!["a"],
            vec![ColumnValue::datetime(types::DateTime {
                year: 2020,
                month: 1,
                day: 2,
                hour: 3,
                minute: 4,
                second: 5,
                millisec: 6,
                microsec: 7,
                ..Default::default()
            })],
        )?;

        assert_eq!(v.a, datetime::DateTime(2020, 1, 2, 3, 4, 5, 6, 7));

        Ok(())
    }

    #[test]
    fn with_unknown_field() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: i32,
        }

        // TODO,
        // let v: Foo = de(vec!["a"], vec![ColumnValue::UnknownField(1)])?;

        // assert_eq!(v.a, 1);

        Ok(())
    }

    #[test]
    fn with_multiple() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: bool,
            b: i64,
            c: i64,
            d: String,
        }

        let v: Foo = de(
            vec!["a", "b", "c", "d"],
            vec![
                ColumnValue::bool_val(true),
                ColumnValue::integer(1),
                ColumnValue::id(2),
                ColumnValue::str(b"3".to_vec()),
            ],
        )?;

        assert!(v.a);
        assert_eq!(v.b, 1);
        assert_eq!(v.c, 2);
        assert_eq!(v.d, "3");

        Ok(())
    }

    #[test]
    fn with_unit() -> Result<(), Box<dyn std::error::Error>> {
        de::<()>(vec!["a"], vec![ColumnValue::bool_val(true)])?;

        Ok(())
    }

    #[test]
    fn with_option() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Foo {
            a: Option<bool>,
        }

        let v: Foo = de(vec!["a"], vec![ColumnValue::bool_val(true)])?;

        assert_eq!(v.a, Some(true));

        Ok(())
    }
}
