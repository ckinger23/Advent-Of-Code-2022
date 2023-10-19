use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args);
    let content = read_contents(file_path);
    let (elf, calories) = loop_through_content(content);
    println!("Elf number: {} had the most with {} calories in his pack", elf, calories);
}

fn parse_args(args: &[String]) -> &str {
    let file_path = &args[1];

    println!("in file {}", file_path);

    return &file_path;
}

fn read_contents(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("should have been able to read in file");
    return content;
}

fn loop_through_content(content : String) -> (i32, i32) {
    let mut elf = 1;
    let mut calories = 0;
    let mut highest_value = 0;
    let mut highest_elf = 1;
    for line in content.lines() {
        if !line.is_empty() {
            let calorie_string = line.to_string();
            let calorie_int: i32 = calorie_string.parse().unwrap();
            calories = calories + calorie_int;
        } else {
            if calories > highest_value {
                highest_value = calories;
                highest_elf = elf;
            }
            elf = elf + 1;
            calories = 0;
        }
    }
    return (highest_elf, highest_value);
}
