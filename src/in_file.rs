use std::io::Read;
use std::{fs::File, io::BufReader};

pub fn f_ln(file: &String) -> std::io::Result<()> {
    // classic file line count
    let file = File::open(&file)?;

    let mut buf_reader = BufReader::new(file);

    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    let mut ln_count: usize = 0;

    for _ in content.lines() {
        ln_count += 1;
    }

    println!("Number of lines: {}\n", ln_count);

    Ok(())
}
