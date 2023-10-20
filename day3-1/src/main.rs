use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args);
    let content = read_contents(file_path);
    let total_score = loop_through_content(content);
    println!("{:?}", total_score);
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

fn loop_through_content(content: String) -> u32 {
    let mut total_score = 0;
    let mut quantity = 0;
    for line in content.lines() {
        let char_count = line.chars().count();
        let halfway = char_count / 2;
        let mut first_half = HashSet::new();
        for (i, c) in line.chars().enumerate() {
            if i < halfway {
                first_half.insert(c);
            } else {
                if first_half.contains(&c) {
                    total_score += calculate_duplicates(c);
                    quantity += 1;
                    break;
                }
            }
        }
    }
    println!("{}", quantity);
    return total_score;
}

fn calculate_duplicates(duplicate: char) -> u32 {
    println!("{}", duplicate);
    let value_as_u32 = duplicate as u32;
    if duplicate.is_ascii_uppercase() {
        println!("{}", value_as_u32 - 38);
        return value_as_u32 - 38;
    } else {
        println!("{}", value_as_u32 - 96);
        return value_as_u32 - 96;
    }
}
