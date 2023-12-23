use std::io::{BufReader, BufRead};
use std::fs::File;

fn check_if_valid(num: u32, color: &str) -> bool {
    if num <= 12 && color == "red" {
        return true;
    }
    else if num <= 13 && color == "green" {
        return true;
    }
    else if num <= 14 && color == "blue" {
        return true;
    }
    return false;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for (line_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(": ").collect();
        let mut line: String = line[1].to_string();
        line = line.replace(", ", " ").replace("; ", " ");
        
        let data: Vec<&str> = line.split(" ").collect();
        let mut line_ok: bool = true;
        for i in (0..data.len()).step_by(2){
            if !check_if_valid(data[i].parse().unwrap(), data[i+1]) {
                line_ok = false;
                break;
            }
        }
        if line_ok {
            sum += line_index + 1;
        }
    }
    println!("{sum}");
}
