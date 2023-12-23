use std::fs::File;
use std::io::{BufReader, BufRead};

fn match_digit(str: &str) -> i32 {
    let mut digit = match &str[..3] {
        "one" => 1,
        "two" => 2,
        "six" => 6,
        _ => -1
    };
    if digit == -1{
        digit = match &str[..4]{
            "zero" => 0,
            "four" => 4,
            "five" => 5,
            "nine" => 9,
            _ => -1
        };
    }
    if digit == -1{
        digit = match &str[..5]{
            "three" => 3,
            "seven" => 7,
            "eight" => 8,
            _ => -1
        };
    }
    return digit

}


fn is_digit_word(str: &str) -> (bool, i32) {
    for num in ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"] {
        if str.starts_with(num){
            return (true, match_digit(str));
        }
    }
    return (false, 0);
}

fn extract_digit(str: &str, first: bool) -> u32 {
    let characters: Vec<char> = str.chars().collect();
    let length = str.len();
    let range: Box<dyn Iterator<Item = usize>>;
    if first {
        range = Box::new(0..length);
    }
    else {
        range = Box::new((0..length).rev());
    }

    for i in range{
        if characters[i].is_digit(10){
            return characters[i].to_digit(10).unwrap();
        }
        let (idw, digit_w) = is_digit_word(&str[i..]);
        if idw {
            return digit_w.try_into().unwrap();
        }
    }
    return 0;
}

fn main() {
    let file_path: &str = "input";
    let file: File = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines(){
        let line = line.unwrap();
        let num: u32 = format!("{}{}", extract_digit(&line, true), extract_digit(&line, false)).parse().unwrap();
        sum+=num;
    }
    println!("{sum}");
}
