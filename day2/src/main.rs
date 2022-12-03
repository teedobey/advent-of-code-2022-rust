use std::io;
use std::collections::hash_map::HashMap;

fn main() {
    part2();
}

fn part1() {
    let mut scores : HashMap<&str, u64> = HashMap::new();
    scores.insert("X", 1); // rock
    scores.insert("Y", 2); // paper
    scores.insert("Z", 3); // scissors
    scores.insert("A X", 3); // rock-rock
    scores.insert("B Y", 3); // paper-paper
    scores.insert("C Z", 3); // scissors-scissors
    scores.insert("A Y", 6); // rock-paper
    scores.insert("B Z", 6); // paper-scissors
    scores.insert("C X", 6); // scissors-rock
    scores.insert("A Z", 0); // rock-scissor
    scores.insert("B X", 0); // paper-rock
    scores.insert("C Y", 0); // scissors-paper

    let mut lines = io::stdin().lines();
    let mut total_score : u64 = 0;

    while let Some(rline) = lines.next() {
        let line = rline.unwrap();
        total_score += scores.get(&line[..]).unwrap(); // for game result
        total_score += scores.get(&line[2..3]).unwrap();
    }

    println!("Total score: {}", total_score);
}

fn part2() {
       
    let mut scores : HashMap<&str, u64> = HashMap::new();
    scores.insert("X", 0); // lose
    scores.insert("Y", 3); // draw
    scores.insert("Z", 6); // win
    scores.insert("A X", 3); // rock-[scissors]
    scores.insert("B Y", 2); // paper-[paper]
    scores.insert("C Z", 1); // scissors-[rock]
    scores.insert("A Y", 1); // rock-[rock]
    scores.insert("B Z", 3); // paper-[scissors]
    scores.insert("C X", 2); // scissors-[paper]
    scores.insert("A Z", 2); // rock-[paper]
    scores.insert("B X", 1); // paper-[rock]
    scores.insert("C Y", 3); // scissors-[scissors]

    let mut lines = io::stdin().lines();
    let mut total_score : u64 = 0;

    while let Some(rline) = lines.next() {
        let line = rline.unwrap();
        total_score += scores.get(&line[..]).unwrap(); 
        total_score += scores.get(&line[2..3]).unwrap();  // for game result
    }

    println!("Total score: {}", total_score); 
}

