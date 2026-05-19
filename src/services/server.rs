use crate::domain::content::Content;
use crate::services::index::recovery_content_by_name;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;

pub fn server_init(index_map: &mut HashMap<String, Content>) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 128];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("{}", request);
        if request.starts_with("GET?name=") {
            let name = request.replace("GET?name=", "").trim().to_string();
            let content = recovery_content_by_name(index_map, &name);
            let response = format!("{{\"name\":\"{}\",\"age\":{}}}", content.name, content.age);
            stream.write(response.as_bytes()).unwrap();
        }
    }
}
