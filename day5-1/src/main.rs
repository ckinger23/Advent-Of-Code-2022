use std::env;
use std::fs;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (file_stacks, file_moves) = parse_args(&args);
    let stacks_content = read_contents(file_stacks);
    let moves_content = read_contents(file_moves);
    let stack = loop_through_stacks(stacks_content);
    let final_string = loop_through_moves(stack, moves_content);
    println!("{}", final_string);
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let file_stacks = &args[1];
    let file_moves = &args[2];

    return (&file_stacks, &file_moves);
}

fn read_contents(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("should have been able to read in file");
    return content;
}

fn loop_through_stacks(content: String) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    for _ in 0..9 {
        stacks.push(Vec::new());
    }
    for line in content.lines() {
       for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
           let second = chunk.nth(1).unwrap();
           if second.is_alphabetic() {
               stacks[idx].insert(0, second);
           }
       }
    }
    return stacks;
}

fn loop_through_moves(mut stacks: Vec<Vec<char>>, moves: String) -> String {
    for line in moves.lines() {
        let parts = line.strip_prefix("move ").unwrap();
        let (amount, rest) = parts.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        let amount = amount.parse().ok().unwrap();
        let from = from.parse::<usize>().ok().unwrap() - 1;
        let to = to.parse::<usize>().ok().unwrap() - 1;
        for _ in 0..amount {
            let move_val = stacks[from].pop().unwrap();
            stacks[to].push(move_val);
        }
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}
