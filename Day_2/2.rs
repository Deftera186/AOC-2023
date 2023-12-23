use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(": ").collect();
        let mut line: String = line[1].to_string();
        line = line.replace(", ", " ").replace("; ", " ");
        
        let data: Vec<&str> = line.split(" ").collect();

        let mut reds: Vec<u32> = vec![];
        let mut greens: Vec<u32> = vec![];
        let mut blues: Vec<u32> = vec![];

        for i in (0..data.len()).step_by(2){
            match data[i+1] {
                "green" => greens.push(data[i].parse().unwrap()),
                "red" => reds.push(data[i].parse().unwrap()),
                "blue" => blues.push(data[i].parse().unwrap()),
                _ => todo!()
            }
        }
        let power = reds.iter().max().unwrap() * greens.iter().max().unwrap() * blues.iter().max().unwrap();
        sum+= power;
    }
    
    println!("{sum}");
}