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

fn loop_through_content(content: String) -> i32 {
    let mut total_score = 0;
    for line in content.lines() {
        let game: Vec<&str> = line.split(' ').collect();
        let opposition = game[0];
        let my_pick = game[1];
        match my_pick {
            "X" => match opposition {
                "A" => total_score += 4,
                "B" => total_score += 1,
                "C" => total_score += 7,
                _ => println!("opposition incorrect choice"),
            },
            "Y" => match opposition {
                "A" => total_score += 8,
                "B" => total_score += 5,
                "C" => total_score += 2,
                _ => println!("opposition incorrect choice"),
            },
            "Z" => match opposition {
                "A" => total_score += 3,
                "B" => total_score += 9,
                "C" => total_score += 6,
                _ => println!("opposition incorrect choice"),
            },
            _ => {
                println!("Incorrect choice in the input text");
            }
        }
    }
    return total_score;
}
