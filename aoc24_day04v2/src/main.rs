use std::fs;
use aoc24_day04v2;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("aoc24_day04/input.txt")
            .expect("Failed to read test file");
    let t0 = Instant::now();
    let p1_ans = aoc24_day04v2::part1(&input);
    let duration = t0.elapsed();
    println!("{p1_ans} done in {:.4?}", duration);

    let t0 = Instant::now();
    let p2_ans = aoc24_day04v2::part2(&input);
    let duration = t0.elapsed();
    println!("{p2_ans} done in {:.4?}", duration);

}
