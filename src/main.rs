use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_name = "word.txt";

    let mut lines = Vec::new();

    let file = File::open(file_name)?;
    for line in io::BufReader::new(file).lines() {
        lines.push(line?);
    }

    let random_index = rand::thread_rng().gen_range(0..lines.len());

    println!("");
    println!("{}", &lines[random_index]);
    println!("");

    Ok(())
}
