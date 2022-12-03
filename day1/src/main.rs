use std::io;
use std::collections::BinaryHeap;

fn main() {                                                              
    let mut lines = io::stdin().lines();
    let mut heap: BinaryHeap<u64> = BinaryHeap::new();
    let mut cals: u64 = 0;

    while let Some(rline) = lines.next() {
        let line = rline.unwrap();
        if line.is_empty() {
            heap.push(cals);
            cals = 0;
        } else {
            cals += line.parse::<u64>().unwrap();
        }
    }
    heap.push(cals);

    println!("Top cals: {}", heap.peek().unwrap());
    let mut top3 : u64 = 0;
    for _ in 0..3 {
        top3 += heap.pop().unwrap();
    }
    println!("Top 3 cumulative: {}", top3);
}
