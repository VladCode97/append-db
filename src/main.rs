pub mod domain;
mod services;

use crate::domain::content::Content;
use crate::services::index::recovery_content_by_name;
use crate::services::index::{edit_age_by_name, index_hash};
use crate::services::put::put_data_in_file;
use crate::services::terminal::in_data;
use std::collections::HashMap;
use std::thread;

fn main() {
    let name_file: &str = "data.ndjson";
    let mut index_map: HashMap<String, Content> = HashMap::new();
    loop {
        println!("\nOPTIONS");
        println!("1. Write on file");
        println!("2. Recovery content by name");
        println!("3. Edit age by name");
        println!("4. Simulate concurrent writes");
        println!("5. Close");
        let option = in_data("option");
        match option.as_str() {
            "1" => {
                let name: String = in_data("name");
                let age: u8 = in_data("age").parse::<u8>().unwrap();
                let content = Content { name, age };
                put_data_in_file(name_file, &content);
                index_hash(&mut index_map, &content);
                println!("Data saved!");
            }
            "2" => {
                let name: String = in_data("name");
                let content = recovery_content_by_name(&mut index_map, &name);
                println!("{:?}", content);
            }
            "3" => {
                let name: String = in_data("name");
                let age: u8 = in_data("age").parse::<u8>().unwrap();
                edit_age_by_name(&mut index_map, &name_file, &name, age);
            }
            "4" => {
                let file_1 = name_file.to_string();
                let file_2 = name_file.to_string();
                let t1 = thread::spawn(move || {
                    let content = Content {
                        name: "Luis".to_string(),
                        age: 29,
                    };
                    println!("THREAD 1 WRITING...");
                    put_data_in_file(&file_1, &content);
                    println!("THREAD 1 DONE");
                });
                let t2 = thread::spawn(move || {
                    let content = Content {
                        name: "Judith".to_string(),
                        age: 30,
                    };
                    println!("THREAD 2 WRITING...");
                    put_data_in_file(&file_2, &content);
                    println!("THREAD 2 DONE");
                });
                t1.join().unwrap();
                t2.join().unwrap();
            }
            "5" => {
                print!("Closing .....");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}
