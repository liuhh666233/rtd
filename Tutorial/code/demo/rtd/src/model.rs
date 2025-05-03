use crate::utils::{
    str_to_timestamp, timestamp_to_datetime_string, timestamp_to_raw_string, ParseItemError,
};
use std::str::FromStr;

const ITEM_COUNT: usize = 7;
const COMMA_FAKE: &str = "<@^_fake_comma_$#>";
const NEWLINE_FAKE: &str = "<@^_fake_newline_$#>";

#[derive(Debug, Clone)]
pub struct Item {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) completed: bool,
    pub(crate) deleted: bool,
    pub(crate) created_at: Option<i64>,
    pub(crate) completed_at: Option<i64>,
    pub(crate) deleted_at: Option<i64>,
}

impl Item {
    // Associated Functions
    pub fn new(
        id: u32,
        name: &str,
        completed: bool,
        deleted: bool,
        created_at: Option<i64>,
        completed_at: Option<i64>,
        deleted_at: Option<i64>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            completed,
            deleted,
            created_at,
            completed_at,
            deleted_at,
        }
    }

    // Methods
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn to_prettier_string(&self) -> String {
        let created_at = timestamp_to_datetime_string(self.created_at);
        let completed_at = timestamp_to_datetime_string(self.completed_at);
        let deleted_at = timestamp_to_datetime_string(self.deleted_at);

        let mut result = format!(
            "{:3} {} {} {}\n\n",
            self.id, // id, 占据3个字符
            if self.completed {
                "\u{2705}" // ✅
            } else {
                "\u{1f532}" // 🔲
            },
            if self.deleted { "\u{1f6ae}" } else { "" }, // 🚮
            self.name,                                   // 任务名称
        );

        if !created_at.is_empty() {
            result.push_str(&format!("\tCreated at: {}\n", created_at));
        }
        if !completed_at.is_empty() {
            result.push_str(&format!("\tCompleted at: {}\n", completed_at));
        }
        if !deleted_at.is_empty() {
            result.push_str(&format!("\tDeleted at: {}\n", deleted_at));
        }
        result
    }
}

/// Serialization
impl ToString for Item {
    fn to_string(&self) -> String {
        let created_at = timestamp_to_raw_string(self.created_at);
        let completed_at = timestamp_to_raw_string(self.completed_at);
        let deleted_at = timestamp_to_raw_string(self.deleted_at);
        // 替换特殊字符
        let name = self
            .name
            .replace(',', COMMA_FAKE)
            .replace(r"\n", NEWLINE_FAKE);

        format!(
            "{},{},{},{},{},{},{}",
            self.id, name, self.completed, self.deleted, created_at, completed_at, deleted_at,
        )
    }
}

/// Deserialization
impl FromStr for Item {
    type Err = ParseItemError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let splited = s.split(',').collect::<Vec<_>>();
        if splited.len() != ITEM_COUNT {
            return Err(ParseItemError(format!(
                "Expected {} properties, found {}",
                ITEM_COUNT,
                splited.len()
            )));
        }

        let id = splited[0].parse::<u32>()?;
        let name = &splited[1]
            .replace(COMMA_FAKE, ",")
            .replace(NEWLINE_FAKE, "\n");

        let completed = splited[2].parse::<bool>()?;
        let deleted = splited[3].parse::<bool>()?;

        let created_at = str_to_timestamp(splited[4])?;
        let completed_at = str_to_timestamp(splited[5])?;
        let deleted_at = str_to_timestamp(splited[6])?;

        Ok(Item::new(
            id,
            name,
            completed,
            deleted,
            created_at,
            completed_at,
            deleted_at,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_to_string() {
        let item = Item::new(1, "test", true, false, Some(1714492800), None, None);
        let string = item.to_string();
        assert_eq!(string, "1,test,true,false,1714492800,,");

        let item = Item::new(1, "test,test", true, false, Some(1714492800), None, None);
        let string = item.to_string();
        assert_eq!(
            string,
            "1,test<@^_fake_comma_$#>test,true,false,1714492800,,"
        );
    }

    #[test]
    fn test_item_from_str() {
        let string = "1,test,true,false,1714492800,,";
        let item = Item::from_str(string).unwrap();
        assert_eq!(item.id, 1);
        assert_eq!(item.name, "test");
        assert_eq!(item.completed, true);
        assert_eq!(item.deleted, false);
        assert_eq!(item.created_at, Some(1714492800));
        assert_eq!(item.completed_at, None);
        assert_eq!(item.deleted_at, None);

        let string = "1,test<@^_fake_comma_$#>test,true,false,1714492800,,";
        let item = Item::from_str(string).unwrap();
        assert_eq!(item.id, 1);
        assert_eq!(item.name, "test,test");
        assert_eq!(item.completed, true);
        assert_eq!(item.deleted, false);
    }
}
