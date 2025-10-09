use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("aoc24_day05/input.txt")
        .expect("Failed to read input file");
    let t0 = Instant::now();
    let part1_ans = aoc24_day05::part1(&input);
    let duration = t0.elapsed();
    println!("Part 1 total is {part1_ans} and took {:.4?}", duration);

    let t0 = Instant::now();
    let part2_ans = aoc24_day05::part2(&input);
    let duration = t0.elapsed();
    println!("Part 2 total is {part2_ans} in {:.4?}", duration);

    let t0 = Instant::now();
    let p2v2_ans = aoc24_day05::part2v2(&input);
    let duration = t0.elapsed();
    println!("Part 2 total is {p2v2_ans} in {:.4?}", duration);
}
