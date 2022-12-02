use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn read_input() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }
    lines
}

fn problem1(lines: Vec<String>) {
    let values = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3),
    ]);
    let score: i32 = lines
        .into_iter()
        .map(|line| {
            let str: &str = &line[..];
            let value = values.get(str).unwrap();
            value
        })
        .sum();
    println!("Promblem 1 total score: {}", score);
}

fn problem2(lines: Vec<String>) {
    let values = HashMap::from([
        ("A X", 3 + 0),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ]);

    let score: i32 = lines
        .into_iter()
        .map(|line| {
            let str: &str = &line[..];
            let value = values.get(str).unwrap();
            value
        })
        .sum();
    println!("Promblem 2 total score: {}", score);
}

fn main() {
    let lines: Vec<String> = read_input();
    problem1(lines.clone());
    problem2(lines.clone());
}
