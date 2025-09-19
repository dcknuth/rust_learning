use std::fs;

fn main() {
    let input_file = "aoc24_day03/input.txt";
    let input = fs::read_to_string(input_file)
        .expect("Failed to read input file");
    let part1_ans = aoc24_day03::part1(&input);
    println!("Part 1 total is {part1_ans}");

    let part2_ans = aoc24_day03::part2(&input);
    println!("Part 2 total is {part2_ans}");
}
