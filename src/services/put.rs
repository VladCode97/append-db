use crate::domain::content::Content;
use fs2::FileExt;
use std::fs::OpenOptions;
use std::io::Write;
pub fn put_data_in_file(name_file: &str, content: &Content) {
    let json = format!(
        "{{\"name\":\"{}\",\"age\":{}}}\n",
        content.name, content.age
    );
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(name_file)
        .expect("Can't open file");
    file.lock_exclusive().expect("Can't lock file");
    std::thread::sleep(std::time::Duration::from_secs(3));
    let _ = file.write_all(json.as_bytes());
}
