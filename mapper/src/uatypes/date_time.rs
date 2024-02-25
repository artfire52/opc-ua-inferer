use crate::Deserialize;
use crate::Serialize;
use crate::MapperResult;
use std::time::SystemTime;
//Number of 100ns from 01/01/1601 
//this code works for unix machine only since it applies a conversion to win32 SystemTime (different epoch)
/// Datetime struct. The value is used as unix timestamp but conversion is performed for encoding.
///
#[derive(Debug,Clone)]
pub struct DateTime {
    value: i64,
}

//wintime=(unixtime*TO_NANOSECOND)+EPOCH_DIFFERENCE: unixtime in second
const TO_NANOSECOND: i64 = 10000000;
const EPOCH_DIFFERENCE: i64 = 116444736000000000;

impl Serialize for DateTime {
    fn serialize(&self) -> Vec<u8> {
        let windows_time = (self.value * TO_NANOSECOND) + EPOCH_DIFFERENCE;
        windows_time.to_le_bytes().to_vec()
    }
}

impl Deserialize for DateTime {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = i64::deserialize(data)?;
        Ok((
            data,
            DateTime {
                value: DateTime::from_win_to_unix(value),
            },
        ))
    }
}

impl DateTime {
    pub fn new() -> DateTime {
        DateTime { value: 0 }
    }
    pub fn from(date: i64) -> DateTime {
        DateTime { value: date }
    }
    pub fn set(&mut self, date: i64) {
        self.value = date;
    }
    pub fn get(&self) -> &i64 {
        &self.value
    }
    fn from_unix_to_win(unix: i64) -> i64 {
        (unix * TO_NANOSECOND) + EPOCH_DIFFERENCE
    }
    pub fn from_win_to_unix(wind: i64) -> i64 {
        (wind - EPOCH_DIFFERENCE) / TO_NANOSECOND
    }
    pub fn now(&mut self) {
        self.value = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
    }

    pub fn new_now() -> DateTime {
        let value = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        DateTime { value: value }
    }

    pub fn read(&mut self, time: i64) {
        self.value = (time - EPOCH_DIFFERENCE) / TO_NANOSECOND;
    }
}