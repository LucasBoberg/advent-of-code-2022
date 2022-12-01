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

fn main() {
    let mut index: u32 = 0;
    let mut total: HashMap<u32, i32> = HashMap::new();
    let lines: Vec<String> = read_input();
    lines.into_iter().for_each(|line| {
        if !line.eq("") {
            let current = total.get(&index);
            match current {
                Some(x) => total.insert(index, x + line.parse::<i32>().unwrap()),
                None => total.insert(index, line.parse::<i32>().unwrap()),
            };
        } else {
            index += 1;
        }
    });
    println!(
        "Most calorie: {}",
        total.iter().max_by_key(|entry| entry.1).unwrap().1
    );
    let mut total_vec: Vec<i32> = total.values().cloned().collect();
    total_vec.sort();
    total_vec.reverse();
    let top3 = total_vec.iter().take(3);
    let sum: i32 = top3.sum();
    println!("Top 3 total: {}", sum);
}
