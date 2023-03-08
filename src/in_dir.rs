use std::fs::{self};
use std::io::Read;
use std::{fs::File, io::BufReader};

fn path_to_paths(paths: &str) -> std::io::Result<i32> {
    let mut ln_count: i32 = 0;

    for path in fs::read_dir(paths.clone()).unwrap() {
        if path.as_ref().unwrap().path().is_dir() {
            let sub_dir_path_buf = path.as_ref().unwrap().path();
            let sub_dir_path = sub_dir_path_buf.to_str().unwrap();
            let second_ln_count = path_to_paths(sub_dir_path)?;
            ln_count += second_ln_count;
        } else {
            let file = File::open(path.unwrap().path())?;
            let mut buf_reader = BufReader::new(file);
            let mut content = String::new();
            buf_reader.read_to_string(&mut content)?;

            for _ in content.lines() {
                ln_count += 1;
            }
        }
    }

    Ok(ln_count)
}


pub fn dir_ln(dir: &String) -> std::io::Result<()> {
    let paths = fs::read_dir(dir).unwrap();

    let mut ln_count = 0;

    for path in paths {
        if path.as_ref().unwrap().path().is_dir() {
            // A very, very long method chain to get 'path' as a string
            ln_count += path_to_paths(
                path.as_ref()
                    .unwrap()
                    .path()
                    .as_mut_os_str()
                    .to_str()
                    .unwrap(),
            ).unwrap_or(0); // uses unwrap_or() in case of non utf-8 files
        } else {
            // Classic line count
            let file = File::open(path.as_ref().unwrap().path())?;

            let mut buf_reader = BufReader::new(file);
            let mut content = String::new();
            buf_reader.read_to_string(&mut content)?;

            for _ in content.lines() {
                ln_count += 1;
            }
        }
    }

    println!("Number of lines: {}\n", ln_count);

    Ok(())
}
