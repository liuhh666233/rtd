use chrono::*;
use std::{
    error::Error,
    fmt::Display,
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};

// 自定义错误类型, 用于解析 Item 时, 如果解析失败, 则返回该错误类型
#[derive(Debug)]
pub struct ParseItemError(pub String);
// 定义 Result 类型, 用于返回解析结果
type Result<T> = std::result::Result<T, ParseItemError>;

// 实现 Error 基本特征, 用于错误处理
impl Error for ParseItemError {}

// 实现 Display 特征, 用于打印错误信息
impl Display for ParseItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Deserialization todo fail: {}", self.0)
    }
}

// 实现 From<ParseIntError> 特征, 用于将 ParseIntError 转换为 ParseItemError
impl From<ParseIntError> for ParseItemError {
    fn from(value: ParseIntError) -> Self {
        Self(value.to_string())
    }
}

// 实现 From<ParseBoolError> 特征, 用于将 ParseBoolError 转换为 ParseItemError
impl From<ParseBoolError> for ParseItemError {
    fn from(value: ParseBoolError) -> Self {
        Self(value.to_string())
    }
}

// TODO(xiaobo): 如果Item中新增其他数据类型,也需要定义新的错误类型转换函数

/// 将时间戳转换为日期时间字符串
pub fn timestamp_to_datetime_string(timestamp: Option<i64>) -> String {
    timestamp.map_or(String::new(), |time_stamp| {
        NaiveDateTime::from_timestamp_opt(time_stamp, 0).map_or(String::new(), |utc| {
            Local
                .from_utc_datetime(&utc)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        })
    })
}

/// 将时间戳转换为原始字符串
pub fn timestamp_to_raw_string(timestamp: Option<i64>) -> String {
    timestamp.map_or(String::new(), |time_stamp| time_stamp.to_string())
}

/// 将字符串转换为时间戳,注意这里返回的是重载的 Result 类型, 而不是 Option 类型
pub fn str_to_timestamp(s: &str) -> Result<Option<i64>> {
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.parse::<i64>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_to_datetime_string() {
        let timestamp = Some(1714492800);
        let datetime_string = timestamp_to_datetime_string(timestamp);
        assert_eq!(datetime_string, "2024-05-01 00:00:00");
    }

    #[test]
    fn test_timestamp_to_raw_string() {
        let timestamp = Some(1714492800);
        let raw_string = timestamp_to_raw_string(timestamp);
        assert_eq!(raw_string, "1714492800");

        let timestamp = None;
        let raw_string = timestamp_to_raw_string(timestamp);
        assert_eq!(raw_string, String::new());
    }

    #[test]
    fn test_str_to_timestamp() {
        let s = "1714492800";
        let timestamp = str_to_timestamp(s).unwrap();
        assert_eq!(timestamp, Some(1714492800));

        let s = "";
        let timestamp = str_to_timestamp(s).unwrap();
        assert_eq!(timestamp, None);

        let s = "not a number";
        let timestamp = str_to_timestamp(s).unwrap_err();
        assert_eq!(
            timestamp.to_string(),
            "Deserialization todo fail: invalid digit found in string\n"
        );
    }
}
