use std::fs;
use std::io::Read;
use std::{fs::File, io::BufReader};

pub fn f_ln(file: &String) -> std::io::Result<()> {
    // Create a default file used as reference
    if !std::path::Path::new("./foo.txt").exists() {
        fs::write("./foo.txt", b"")?
    }

    let file = File::open(file).unwrap_or(File::open("./foo.txt").unwrap()); // Use the default file if the chosen file isn't valid

    // Classic line count
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap_or(0);

    let mut ln_count: usize = 0;

    for _ in content.lines() {
        ln_count += 1;
    }

    if ln_count == 0 {
        panic!("Couldn't read file");
    } else {
        println!("Number of lines: {}\n", ln_count);
    }

    Ok(())
}
