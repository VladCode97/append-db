use crate::domain::content::Content;
use crate::services::put::put_data_in_file;
use std::collections::HashMap;

pub fn index_hash(index_map: &mut HashMap<String, Content>, content: &Content) {
    index_map.insert(content.name.to_string(), {
        Content {
            name: content.name.to_string(),
            age: content.age,
        }
    });
}

pub fn recovery_content_by_name(index_map: &mut HashMap<String, Content>, name: &str) -> Content {
    match index_map.get(name) {
        Some(content) => content.clone(),
        None => {
            panic!("No content found");
        }
    }
}

pub fn edit_age_by_name(
    index_map: &mut HashMap<String, Content>,
    name_file: &str,
    name: &str,
    age: u8,
) {
    match index_map.get_mut(name) {
        Some(content) => {
            content.age = age;
            put_data_in_file(name_file, content);
        }
        None => {
            panic!("No content found");
        }
    }
}
