use std::{fs, io::BufReader};
use std::io::prelude::*;

fn get_first_digit(str: &String) -> u32 {
    for c in str.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    return 0;
}

fn get_last_digit(str: &String) -> u32 {
    for c in str.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    return 0;
}

fn main() {
    let file_path: &str = "input";
    let file: fs::File = fs::File::open(file_path).unwrap();
    let reader: BufReader<fs::File> = BufReader::new(file);
    let mut sum: u32 = 0; 

    for line in reader.lines() {
        let line = line.unwrap();
        let first_digit: u32 = get_first_digit(&line);
        let last_digit: u32 = get_last_digit(&line);
        let line_sum: u32 = format!("{first_digit}{last_digit}").parse().unwrap();
        sum += line_sum;
    }
    
    println!("{sum}");
}
