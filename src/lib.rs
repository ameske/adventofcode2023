use std::fs;
use std::io;
use std::io::Read;

pub fn run_day(day: u64, part1: fn(&str) -> i64, part2: fn(&str) -> i64) {
    let input = load_problem_input(day).unwrap();
    println!("Day {} Part 1: {}", day, part1(&input));
    println!("Day {} Part 2: {}", day, part2(&input));
}

pub fn load_problem_input(day: u64) -> io::Result<String> {
    let path = format!("inputs/day{}.txt", day);
    load_input(&path)
}

pub fn load_test_input(day: u64) -> io::Result<String> {
    let path = format!("inputs/day{}_test.txt", day);
    load_input(&path)
}

fn load_input(path: &str) -> io::Result<String> {
    let mut fd = fs::File::open(path)?;

    let mut s = String::new();

    fd.read_to_string(&mut s)?;

    Ok(s)
}
