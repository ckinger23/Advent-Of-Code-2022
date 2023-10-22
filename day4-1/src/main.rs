use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args);
    let content = read_contents(file_path);
    let total_score = loop_through_content(content);
    println!("{}", total_score);
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
    for line in content.lines() {
        let pairs = line.split(',').collect::<Vec<&str>>();
        let elf1 = pairs[0];
        let elf2 = pairs[1];
        let elf1_values = elf1.split('-').collect::<Vec<&str>>();
        let elf2_values = elf2.split('-').collect::<Vec<&str>>();
        let e1_v1 = elf1_values[0].parse::<i32>().unwrap();
        let e1_v2 = elf1_values[1].parse::<i32>().unwrap();
        let e2_v1 = elf2_values[0].parse::<i32>().unwrap();
        let e2_v2 = elf2_values[1].parse::<i32>().unwrap();
        if (e1_v1 <= e2_v1) && (e1_v2 >= e2_v2) {
            total_score += 1;
            println!("{} option 1", line);
        } else if (e2_v1 <= e1_v1) && (e2_v2 >= e1_v2) {
            total_score += 1;
            println!("{} option 2", line);
        }
    }
    return total_score;
}
