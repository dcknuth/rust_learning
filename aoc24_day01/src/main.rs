use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    // Prep, read the input file
    //let input_file = "aoc24_day01/test.txt";
    let input_file = "aoc24_day01/input.txt";
    let contents = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    
    // start timing
    let t1 = Instant::now();

    // Put each column into a vector
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        list1.push(nums[0]);
        list2.push(nums[1]);
    }
    
    //Part 1
    // Let's sort both lists
    list1.sort();
    list2.sort();

    // Then let's find the abs difference between each pair and add them to
    //  a total
    let mut total = 0;
    for i in 0..list1.len() {
        total += (list1[i] - list2[i]).abs();
    }

    // end timing
    let duration = t1.elapsed();
    // Print the amount
    println!("Part 1: The total distance apart is: {total}");
    println!("Execution time: {:.4?}", duration);

    // Part 2
    let t1 = Instant::now();
    let mut sim_score = 0;
    let mut counts = HashMap::new();
    for i in list2 {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }
    for i in list1 {
        sim_score += i * counts.get(&i).copied().unwrap_or(0);
    }
    let duration = t1.elapsed();
    println!("Part 2: Similarity score is {sim_score}");
    println!("Execution time: {:.4?}", duration);
}
