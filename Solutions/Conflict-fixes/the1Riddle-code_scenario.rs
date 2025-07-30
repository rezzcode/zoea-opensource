/**
 * A program that takes a value, eg 1 and returns its type
 * plus other types
 * 
 * Note: the program does not accept strings
 * 
 * How to use: 
 * rezzcode@linux$ rustc get_type.rs
 * rezzcode@linux$ ./get_type.rs 4
 * 
 * 4 is of type i32: 4 bytes
 * Other types:
 * Size of char: 4 bytes
 * Size of u32: 4 bytes
 * Size of f64: 8 bytes
 */

use std::env; // For accessing command-line arguments
use std::mem; // For retrieving size of types in bytes

fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Ensure exactly one argument (besides program name) is provided
    if args.len() != 2 {
        eprintln!("Please pass in a value: Usage: {} <value>", args[0]);
        return;
    }

    let input = &args[1]; // The input string to be parsed

    // Try parsing input as i32
    if let Ok(value) = input.parse::<i32>() {
        println!("{value} is of type i32: {} bytes", mem::size_of::<i32>());
        print_other_types("i32");
    }
    // Try parsing input as u32
    else if let Ok(value) = input.parse::<u32>() {
        println!("{value} is of type u32: {} bytes", mem::size_of::<u32>());
        print_other_types("u32");
    }
    // Try parsing input as f64
    else if let Ok(value) = input.parse::<f64>() {
        println!("{value} is of type f64: {} bytes", mem::size_of::<f64>());
        print_other_types("f64");
    }
    // If parsing as number fails, check if it's a single character
    else if input.chars().count() == 1 {
        let c = input.chars().next().unwrap(); // Safe unwrap after count check
        println!("{c} is of type char: {} bytes", mem::size_of::<char>());
        print_other_types("char");
    }
    // If all parsing attempts fail, print an error
    else {
        println!("Could not determine type of input: {}", input);
    }
}

// Print sizes of types excluding the one that matched
fn print_other_types(exclude: &str) {
    println!("Other types:");
    if exclude != "char" {
        println!("Size of char: {} bytes", mem::size_of::<char>());
    }
    if exclude != "i32" {
        println!("Size of i32: {} bytes", mem::size_of::<i32>());
    }
    if exclude != "u32" {
        println!("Size of u32: {} bytes", mem::size_of::<u32>());
    }
    if exclude != "f64" {
        println!("Size of f64: {} bytes", mem::size_of::<f64>());
    }
}
// version A for the1Riddle
