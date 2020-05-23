use std::fs::File;
use std::io;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

#[test]
fn main() -> io::Result<()> {
    let path: &str = "lines.txt";

    // let mut output: File = File::create(path)?;
    // write!(output, "Rust\n:)\nFun");

    let input: File = File::open(path)?;
    let buffered: BufReader<File> = BufReader::new(input);
    
    let text = fs::read_to_string(path).unwrap();
    println!("{}",text);


    for line in buffered.lines().map(|x| x.unwrap()) {
        // line: String     x:Result<String, Error>
        println!("{}", line);
    }

    Ok(())
}