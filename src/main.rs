use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Error: Expected 4 arguments, but got {}. Usage: {} <input file> <output file> <key>", args.len(), args[0]);
        return;
    }

    let file = fs::read(&args[1]).expect("Error: Unable to read file.");

    if file.len() == 0 {
        println!("Error: File is empty.");
        return;
    }
    
    println!("File size: {} bytes", file.len());

    let key = args[3].as_bytes();
    if key.len() == 0 {
        println!("Error: Key is empty.");
        return;
    }

    let mut result = Vec::new();

    for i in 0..file.len() {
        let byte = file[i] ^ key[i % key.len()];
        result.push(byte);
    }

    fs::write(&args[2], &result).expect("Error: Unable to write to file.");

    println!("Done!! Output file: {}", args[2]); 
}