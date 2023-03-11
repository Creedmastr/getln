#![feature(path_as_mut_os_str)]
#![feature(file_create_new)]
#![feature(const_mut_refs)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod in_dir;
mod in_file;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let now = Instant::now();

    let args: Vec<String> = std::env::args().collect();

    if std::fs::metadata(&args[1]).unwrap().is_dir() {
        // Handles Windows '\' , mostly when using tab completion
        in_dir::dir_ln(&args[1].replace(r"\", "/")).expect("Couldn't read dir");
    } else {
        in_file::f_ln(&args[1].replace(r"\", "/")).expect("Couldn't read file");
    }

    println!("Finished in: {:.2?}", now.elapsed());

    Ok(())
}
