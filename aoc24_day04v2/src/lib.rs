use rayon::prelude::*;

fn load_matrix(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn look_up(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if y < 3 {
        return false;
    }
    if m[y-1][x] != 'M' {
        return false;
    }
    if m[y-2][x] != 'A' {
        return false;
    }
    if m[y-3][x] != 'S' {
        return false;
    }
    return true;
}

fn look_down(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if y > m.len() - 4 {
        return false;
    }
    if m[y+1][x] != 'M' {
        return false;
    }
    if m[y+2][x] != 'A' {
        return false;
    }
    if m[y+3][x] != 'S' {
        return false;
    }
    return true;
}

fn look_left(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if x < 3 {
        return false;
    }
    if m[y][x-1] != 'M' {
        return false;
    }
    if m[y][x-2] != 'A' {
        return false;
    }
    if m[y][x-3] != 'S' {
        return false;
    }
    return true;
}

fn look_right(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if x > m[0].len() -4 {
        return false;
    }
    if m[y][x+1] != 'M' {
        return false;
    }
    if m[y][x+2] != 'A' {
        return false;
    }
    if m[y][x+3] != 'S' {
        return false;
    }
    return true;
}

fn look_up_left(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if x < 3 || y < 3 {
        return false;
    }
    if m[y-1][x-1] != 'M' {
        return false;
    }
    if m[y-2][x-2] != 'A' {
        return false;
    }
    if m[y-3][x-3] != 'S' {
        return false;
    }
    return true;
}

fn look_up_right(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if x > m[0].len() - 4 || y < 3 {
        return false;
    }
    if m[y-1][x+1] != 'M' {
        return false;
    }
    if m[y-2][x+2] != 'A' {
        return false;
    }
    if m[y-3][x+3] != 'S' {
        return false;
    }
    return true;
}

fn look_down_left(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if x < 3 || y > m.len() - 4 {
        return false;
    }
    if m[y+1][x-1] != 'M' {
        return false;
    }
    if m[y+2][x-2] != 'A' {
        return false;
    }
    if m[y+3][x-3] != 'S' {
        return false;
    }
    return true;
}

fn look_down_right(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    if x > m[0].len() - 4 || y > m.len() - 4 {
        return false;
    }
    if m[y+1][x+1] != 'M' {
        return false;
    }
    if m[y+2][x+2] != 'A' {
        return false;
    }
    if m[y+3][x+3] != 'S' {
        return false;
    }
    return true;
}

fn search_from(m: &Vec<Vec<char>>, y: usize, x: usize) -> u32 {
    let mut count = 0;
    if look_up(m, y, x) {
        count += 1;
    }
    if look_down(m, y, x) {
        count += 1;
    }
    if look_left(m, y, x) {
        count += 1;
    }
    if look_right(m, y, x) {
        count += 1;
    }
    if look_up_left(m, y, x) {
        count += 1;
    }
    if look_up_right(m, y, x) {
        count += 1;
    }
    if look_down_left(m, y, x) {
        count += 1;
    }
    if look_down_right(m, y, x) {
        count += 1;
    }

    count
}

pub fn part1(s: &str) -> u32 {
    let m = load_matrix(s);
    
    let count = m.par_iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, c)| {
            if *c == 'X' {
                search_from(&m, y, x)
            } else {
                0
            }
        }).sum::<u32>()
    }).sum::<u32>();

    count
}

pub fn part2(s: &str) -> u32 {
    let m = load_matrix(s);
    let mut count = 0;

    let mut y = 1;
    let mut x = 1;
    while y < m.len() - 1 {
        x = 1;
        while x < m[y].len() - 1 {
            if m[y][x] == 'A' {
                if m[y-1][x-1] == 'M' && m[y+1][x+1] == 'S' &&
                    m[y+1][x-1] == 'M' && m[y-1][x+1] == 'S' {
                    count += 1;   
                } else if m[y-1][x-1] == 'M' && m[y+1][x+1] == 'S' &&
                    m[y+1][x-1] == 'S' && m[y-1][x+1] == 'M' {
                    count += 1;   
                } else if m[y-1][x-1] == 'S' && m[y+1][x+1] == 'M' &&
                    m[y+1][x-1] == 'M' && m[y-1][x+1] == 'S' {
                    count += 1;   
                } else if m[y-1][x-1] == 'S' && m[y+1][x+1] == 'M' &&
                    m[y+1][x-1] == 'S' && m[y-1][x+1] == 'M' {
                    count += 1;   
                }
            }
            x += 1;
        }
        y += 1;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("aoc24_day04/test.txt")
            .expect("Failed to read test file");
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("aoc24_day04/test.txt")
            .expect("Failed to read test file");
        assert_eq!(part2(&input), 9);
    }
}