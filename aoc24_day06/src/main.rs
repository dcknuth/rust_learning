use std::fs;
use std::collections::HashMap;
use aoc24_day06::{part1, part2, part2v2, part2v3};
use std::time::Instant;

fn main() {
    let s = fs::read_to_string("aoc24_day06/input.txt")
        .expect("Could not read input file");

    let t0 = Instant::now();
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    let p1_ans = part1(&s, &mut visited);
    let duration = t0.elapsed();
    println!("{p1_ans} in {:.4?}", duration);

    let t0 = Instant::now();
    let p2_ans = part2(&s);
    let duration = t0.elapsed();
    println!("{p2_ans} in {:.4?}", duration);

    let t0 = Instant::now();
    let p2_ans = part2v2(&s);
    let duration = t0.elapsed();
    println!("{p2_ans} in {:.4?}", duration);

    let t0 = Instant::now();
    let p2_ans = part2v3(&s, &visited);
    let duration = t0.elapsed();
    println!("{p2_ans} in {:.4?}", duration);    
}
