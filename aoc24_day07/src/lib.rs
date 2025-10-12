fn try_ops(result: usize, cur_total: usize, mut num_list: Vec<usize>) -> bool {
    // assuming all numbers are >0
    let cur_num = num_list.pop().unwrap();
    if num_list.len() == 0 {
        if cur_total + cur_num == result || cur_total * cur_num == result {
            return true;
        } else {
            return false;
        }
    } else {
        return try_ops(result, cur_total + cur_num, num_list.clone()) ||
            try_ops(result, cur_total * cur_num, num_list.clone());
    }
}

fn bool_combinations(n: usize) -> impl Iterator<Item = Vec<bool>> {
    (0..1 << n).map(move |i| {
        (0..n).map(move |j| (i >> j) & 1 == 1).collect()
    })
}

fn try_ops_v2(result: usize, mut num_list: Vec<usize>) -> bool {
    // assuming all numbers are >0
    num_list.sort_by(|a, b| b.cmp(a));
    
    for combo in bool_combinations(num_list.len() - 1) {
        let mut total = num_list[0];
        for i in 1..num_list.len() {
            if combo[i-1] {
                total *= num_list[i];
            } else {
                total += num_list[i];
            }
            if total > result {
                break;
            }
        }
        if total == result {
            return true;
        }
    }
    false
}

pub fn part1(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let result: usize = parts[0].parse().unwrap();
        let mut operands: Vec<usize> = parts[1].trim().split_whitespace()
            .map(|op| op.parse().unwrap())
            .rev().collect();

        let cur_num = operands.pop().unwrap();
        if try_ops(result, cur_num, operands) {
            total += result;
        }
    }

    total
}

fn try3ops(result: usize, cur_total: usize, mut num_list: Vec<usize>) -> bool {
    // assuming all numbers are >0
    let cur_num = num_list.pop().unwrap();
    let cur_places = (cur_num as f64).log10().floor() as usize + 1;
    let cur_cat: usize = cur_total * 10usize.pow(cur_places as u32) + cur_num;
    if num_list.len() == 0 {
        if cur_total + cur_num == result || 
            cur_total * cur_num == result ||
            cur_cat == result{
            return true;
        } else {
            return false;
        }
    } else {
        return try3ops(result, cur_total + cur_num, num_list.clone()) ||
            try3ops(result, cur_total * cur_num, num_list.clone()) ||
            try3ops(result, cur_cat, num_list.clone());
    }
}

pub fn part2(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let result: usize = parts[0].parse().unwrap();
        let mut operands: Vec<usize> = parts[1].trim().split_whitespace()
            .map(|op| op.parse().unwrap())
            .rev().collect();

        let cur_num = operands.pop().unwrap();
        if try3ops(result, cur_num, operands) {
            total += result;
        }
    }

    total
}

use rayon::prelude::*;
pub fn part2v2(s: &str) -> usize {
    s.lines().par_bridge().map(|line| {
        let parts: Vec<&str> = line.split(':').collect();
        let result: usize = parts[0].parse().unwrap();
        let mut operands: Vec<usize> = parts[1].trim().split_whitespace()
            .map(|op| op.parse().unwrap())
            .rev().collect();
        let cur_num = operands.pop().unwrap();
        if try3ops(result, cur_num, operands) {
            result
        } else {
            0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use once_cell::sync::Lazy;
    use super::*;

    static TEST_INPUT: Lazy<String> = Lazy::new(|| {
        fs::read_to_string("aoc24_day07/test.txt")
            .expect("Could not read test file")
    });

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 11387);
    }
}
