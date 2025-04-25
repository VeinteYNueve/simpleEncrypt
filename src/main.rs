use std::fs;
// use std::env;
use std::io::{self};

fn main() {
    println!("Welcome to the XOR encrypter!");
    loop {
        println!();
        println!("Please enter the name of the file to encrypt/decrypt or type \"exit\" to leave:");
        
        let mut input_file = String::new();
        io::stdin().read_line(&mut input_file).unwrap();
        let input_file = input_file.trim();
        if input_file == "exit" {
            break;
        }

        let file = fs::read(input_file).expect("Error: Unable to read file.");

        if file.len() == 0 {
            println!("File is empty.");
            continue;
        }
        
        println!();
        println!("File size: {} bytes", file.len());
        println!();

        println!("Type your key here (or type \"exit\" to go back to the start):");
        let mut input_key = String::new();
        io::stdin().read_line(&mut input_key).unwrap();
        let input_key = input_key.trim();
        if input_key == "exit" {
            println!();
            continue;
        }

        let key = input_key.as_bytes();
        if key.len() == 0 {
            println!("Key is empty.");
            continue;
        }

        println!();
        println!("Put the name of your output file here (or type \"exit\" to go back to the start):");
        let mut output_file = String::new();
        io::stdin().read_line(&mut output_file).unwrap();
        let output_file = output_file.trim();
        if output_file == "exit" {
            println!();
            continue;
        }

        let mut result = Vec::new();

        for i in 0..file.len() {
            let byte = file[i] ^ key[i % key.len()];
            result.push(byte);
        }

        fs::write(&output_file, &result).expect("Error: Unable to write to file.");
        
        println!();
        println!("Done!! Output file: {}", output_file); 
    }
    println!();
    println!("Thank you for coming!");
}