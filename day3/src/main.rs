use std::io;

fn main() {
    part2();
}

fn part1() {
    let mut total_score: u64 = 0;
    let mut lines = io::stdin().lines();

    'nextline: while let Some(rline) = lines.next() {
        let mut arrayset: [i32; 52] = [0; 52];
        let letters: Vec<char> = rline.unwrap().chars().collect();

        for i in 0..(letters.len() / 2) {
            arrayset[(priority(letters[i]) - 1) as usize] = 1;
        }
        for i in letters.len() / 2..letters.len() {
            if arrayset[(priority(letters[i]) - 1) as usize] == 1 {
                total_score += priority(letters[i]) as u64;
                continue 'nextline;
            }
        }
    }
    println!("Total score: {}", total_score);
}

fn part2() {
    let mut total_score: u64 = 0;
    let mut lines = io::stdin().lines();
    let mut cum_arrayset: [i32; 52] = [0; 52];
    let mut num = 0;


    while let Some(rline) = lines.next() {
        let mut arrayset: [i32; 52] = [0; 52];
        let letters: Vec<char> = rline.unwrap().chars().collect();

        for i in 0..letters.len() {
            arrayset[(priority(letters[i]) - 1) as usize] = 1;
        }
        for i in 0..52 {
            cum_arrayset[i] += arrayset[i];
        }
        if num % 3 == 2 {
            let index = cum_arrayset.iter().position(|&r| r == 3).unwrap();
            total_score += index as u64 + 1;
            cum_arrayset = [0; 52];
        }        
        num += 1;
    }
    println!("Total score: {}", total_score);
}

fn priority(input: char) -> i32 {
    if input as u32 >= 'a' as u32 {
        return (input as i32) - 'a' as i32 + 1;
    } else {
        return (input as i32) - 'A' as i32 + 27;
    }
}