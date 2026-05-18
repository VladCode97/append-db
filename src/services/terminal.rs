use std::io;

pub fn in_data(field: &str) -> String {
    let mut input: String = String::new();
    if field != "1" || field != "2" || field != "3" || field != "4" {
        println!("Enter your {}: ", field);
    };
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
