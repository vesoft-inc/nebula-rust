/*
datetime.rs 文件中定义了一些日期和时间相关的数据类型以及它们与 chrono 库的集成。以下是文件中定义的主要内容：
类型别名定义：在文件的开头，定义了一系列类型别名，如 Year、Month、Day 等，它们都是整数类型的别名，用于表示日期和时间的各个部分。
结构体 Timestamp：表示一个时间戳，它包含一个 i64 类型的字段，用于存储时间戳的值。此结构体实现了 Deserialize trait，以支持从数据格式中反序列化时间戳。
结构体 YearMonth：表示年份和月份的组合，包含了 Year 和 Month 两个字段。同样，它也实现了 Deserialize trait，以支持从数据格式中反序列化年份和月份。
结构体 Date：表示日期，包含了 Year、Month 和 Day 三个字段。同样，它也实现了 Deserialize trait，以支持从数据格式中反序列化日期。
结构体 Time：表示时间，包含了 Hour、Minute、Second 和 Millisec 四个字段。同样，它也实现了 Deserialize trait，以支持从数据格式中反序列化时间。
结构体 DateTime：表示日期和时间的组合，包含了年、月、日、时、分、秒、毫秒和微秒八个字段。同样，它也实现了 Deserialize trait，以支持从数据格式中反序列化日期和时间。
测试模块：在文件的末尾，包含了一些测试函数，用于测试日期和时间类型的反序列化是否正常工作。这些测试函数使用了 chrono 库来验证反序列化的结果是否与预期一致。

总之，datetime.rs 文件定义了一些日期和时间相关的数据结构，并提供了与 chrono 库的集成，以便在 Rust 项目中方便地处理日期和时间数据。这对于与时间相关的应用程序非常有用。

*/
pub type Year = i16;
pub type Month = i8;
pub type Day = i8;
pub type Hour = i8;
pub type Minute = i8;
pub type Second = i8;
pub type Millisec = i16;
pub type Microsec = i16;

use serde::Deserialize;

//
#[derive(Deserialize, PartialEq, Debug)]
pub struct Timestamp(pub i64);

impl Timestamp {
    #[cfg(feature = "chrono")]
    pub fn to_naive_date_time(&self) -> chrono::NaiveDateTime {
        chrono::NaiveDateTime::from_timestamp_opt(self.0, 0)
            .expect("chrono::NaiveDateTime::from_timestamp_opt")
    }
}

// v1
#[derive(Deserialize, PartialEq, Debug)]
pub struct YearMonth(pub Year, pub Month);

impl YearMonth {
    #[cfg(feature = "chrono")]
    pub fn to_naive_date(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(self.0 as i32, self.1 as u32, 1)
            .expect("chrono::NaiveDate::from_ymd_opt")
    }
}

// v3 v2 v1
#[derive(Deserialize, PartialEq, Debug)]
pub struct Date(pub Year, pub Month, pub Day);

impl Date {
    #[cfg(feature = "chrono")]
    pub fn to_naive_date(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(self.0 as i32, self.1 as u32, self.2 as u32)
            .expect("chrono::NaiveDate::from_ymd_opt")
    }
}

// v3 v2 v1
#[derive(Deserialize, PartialEq, Debug)]
pub struct Time(pub Hour, pub Minute, pub Second, pub Millisec);

impl Time {
    #[cfg(feature = "chrono")]
    pub fn to_naive_date_time(&self) -> chrono::NaiveDateTime {
        let d =
            chrono::NaiveDate::from_ymd_opt(1970, 1, 1).expect("chrono::NaiveDate::from_ymd_opt");
        let t = chrono::NaiveTime::from_hms_milli_opt(
            self.0 as u32,
            self.1 as u32,
            self.2 as u32,
            self.3 as u32,
        )
        .expect("chrono::NaiveTime::from_hms_milli_opt");
        chrono::NaiveDateTime::new(d, t)
    }
}

// v3 v2 v1
#[derive(Deserialize, PartialEq, Debug)]
pub struct DateTime(
    pub Year,
    pub Month,
    pub Day,
    pub Hour,
    pub Minute,
    pub Second,
    pub Millisec,
    pub Microsec,
);

impl DateTime {
    #[cfg(feature = "chrono")]
    pub fn to_naive_date_time(&self) -> chrono::NaiveDateTime {
        let d = chrono::NaiveDate::from_ymd_opt(self.0 as i32, self.1 as u32, self.2 as u32)
            .expect("chrono::NaiveDate::from_ymd_opt");
        let t = chrono::NaiveTime::from_hms_milli_opt(
            self.3 as u32,
            self.4 as u32,
            self.5 as u32,
            self.6 as u32,
        )
        .expect("chrono::NaiveTime::from_hms_milli_opt");
        chrono::NaiveDateTime::new(d, t)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "chrono")]
    use super::*;

    #[cfg(feature = "chrono")]
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[test]
    fn chrono_for_timestamp() {
        #[cfg(feature = "chrono")]
        assert_eq!(
            Timestamp(1577836800).to_naive_date_time(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap()
            )
        );
    }

    #[test]
    fn chrono_for_year_month() {
        #[cfg(feature = "chrono")]
        assert_eq!(
            YearMonth(2020, 1).to_naive_date(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()
        );
    }

    #[test]
    fn chrono_for_date() {
        #[cfg(feature = "chrono")]
        assert_eq!(
            Date(2020, 1, 2).to_naive_date(),
            NaiveDate::from_ymd_opt(2020, 1, 2).unwrap()
        );
    }

    #[test]
    fn chrono_for_time() {
        #[cfg(feature = "chrono")]
        assert_eq!(
            Time(1, 2, 3, 4).to_naive_date_time(),
            NaiveDateTime::new(
                NaiveDate::default(),
                NaiveTime::from_hms_milli_opt(1, 2, 3, 4).unwrap(),
            )
        );
    }

    #[test]
    fn chrono_for_datetime() {
        #[cfg(feature = "chrono")]
        assert_eq!(
            DateTime(2020, 1, 2, 3, 4, 5, 6, 7).to_naive_date_time(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
                NaiveTime::from_hms_milli_opt(3, 4, 5, 6).unwrap(),
            )
        );
    }
}
