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
