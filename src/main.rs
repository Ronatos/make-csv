use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("rawdata.txt")
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);

    let mut contents2 = String::new();

    for (i, c) in contents.chars().enumerate() {
        if c == ' ' {
            let d = ',';
            contents2.push_str(&d.to_string());
        }
        else {
            contents2.push_str(&c.to_string());   
        }
    }

    println!("{}", contents2);

    write!(file, "{}", contents2)?;
    Ok(())
}