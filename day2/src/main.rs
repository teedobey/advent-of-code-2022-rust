use std::io;
use std::collections::hash_map::HashMap;

fn main() {
    part2();
}

fn part1() {
    let mut scores : HashMap<&str, u64> = HashMap::new();
    scores.insert("A X", 3+1); // rock-rock
    scores.insert("B Y", 3+2); // paper-paper
    scores.insert("C Z", 3+3); // scissors-scissors
    scores.insert("A Y", 6+2); // rock-paper
    scores.insert("B Z", 6+3); // paper-scissors
    scores.insert("C X", 6+1); // scissors-rock
    scores.insert("A Z", 0+3); // rock-scissor
    scores.insert("B X", 0+1); // paper-rock
    scores.insert("C Y", 0+2); // scissors-paper

    let mut lines = io::stdin().lines();
    let mut total_score : u64 = 0;

    while let Some(rline) = lines.next() {
        let line = rline.unwrap();
        total_score += scores.get(&line[..]).unwrap();
    }

    println!("Total score: {}", total_score);
}

fn part2() {
       
    let mut scores : HashMap<&str, u64> = HashMap::new();
    scores.insert("A X", 3+0); // rock-[scissors]
    scores.insert("B Y", 2+3); // paper-[paper]
    scores.insert("C Z", 1+6); // scissors-[rock]
    scores.insert("A Y", 1+3); // rock-[rock]
    scores.insert("B Z", 3+6); // paper-[scissors]
    scores.insert("C X", 2+0); // scissors-[paper]
    scores.insert("A Z", 2+6); // rock-[paper]
    scores.insert("B X", 1+0); // paper-[rock]
    scores.insert("C Y", 3+3); // scissors-[scissors]

    let mut lines = io::stdin().lines();
    let mut total_score : u64 = 0;

    while let Some(rline) = lines.next() {
        let line = rline.unwrap();
        total_score += scores.get(&line[..]).unwrap(); 
    }

    println!("Total score: {}", total_score); 
}
