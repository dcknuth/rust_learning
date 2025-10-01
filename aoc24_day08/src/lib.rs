use std::collections::{HashMap, HashSet};

pub fn create_matrix(s: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(i32, i32)>>) {
    let mut m = Vec::new();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c != '.' {
                antennas.entry(c).or_insert(vec![]).push((y as i32, x as i32)); 
            }
        }
        m.push(row);
    }

    (m, antennas)
}

pub fn part1(s: &str) -> usize {
    let (m, antennas) = create_matrix(s);
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    
    for key in antennas.keys() {
        let cells = antennas.get(key).unwrap();
        let mut i = 0;
        while i < cells.len() - 1 {
            let (y1, x1) = cells[i];
            let mut j = i + 1;
            while j < cells.len() {
                let (y2, x2) = cells[j];
                let dy = y2 - y1;
                let dx = x2 - x1;
                // distance +- is on map, add to antinodes
                if y2 + dy > -1 && y2 + dy < m.len() as i32 &&
                    x2 + dx > -1 && x2 + dx < m[0].len() as i32 {
                        antinodes.insert((y2 + dy, x2 + dx));
                }
                if y1 - dy > -1 && y1 - dy < m.len() as i32 &&
                    x1 - dx > -1 && x1 - dx < m[0].len() as i32 {
                        antinodes.insert((y1 - dy, x1 - dx));
                }
                j += 1
            }
            i += 1
        }
    }

    antinodes.len()
}

pub fn part2(s: &str) -> usize {
    let (m, antennas) = create_matrix(s);
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    
    for key in antennas.keys() {
        let cells = antennas.get(key).unwrap();
        let mut i = 0;
        while i < cells.len() - 1 {
            let (y1, x1) = cells[i];
            let mut j = i + 1;
            while j < cells.len() {
                let (y2, x2) = cells[j];
                // add the pair of tower locations
                // these may repeat, but the set will ensure only one
                antinodes.insert((y1, x1));
                antinodes.insert((y2, x2));
                let dy = y2 - y1;
                let dx = x2 - x1;
                // add all distances on map to antinodes
                let mut step = 1;
                while y2 + step * dy > -1 &&
                    y2 + step * dy < m.len() as i32 &&
                    x2 + step * dx > -1 &&
                    x2 + step * dx < m[0].len() as i32 {
                    antinodes.insert((y2 + step * dy, x2 + step * dx));
                    step += 1;
                }
                step = 1;
                while y1 - step * dy > -1 &&
                    y1 - step * dy < m.len() as i32 &&
                    x1 - step * dx > -1 &&
                    x1 - step * dx < m[0].len() as i32 {
                    antinodes.insert((y1 - step * dy, x1 - step * dx));
                    step += 1;
                }
                j += 1
            }
            i += 1
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use once_cell::sync::Lazy;
    use super::*;

    static TEST_INPUT: Lazy<String> = Lazy::new(|| {
        fs::read_to_string("aoc24_day08/test.txt")
        .expect("Could not read test file")
    });

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 34);
    }
}