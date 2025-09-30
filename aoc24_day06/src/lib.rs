use std::collections::HashMap;

pub fn load_matrix(s: &str) -> (Vec<Vec<char>>, Option<(i32, i32)>) {
    let mut m = Vec::new();
    let mut pos = None;

    for (y, line) in s.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                pos = Some((y as i32, x as i32));
            }
            row.push(c);
        }
        m.push(row);
    }
    
    (m, pos)
}

fn turn_right(dir: &mut (i32, i32), c: &mut char) {
    *c = match *c {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Don't know which way to turn"),
    };
    *dir = (dir.1, -dir.0);
}

fn move_or_turn(m: &mut Vec<Vec<char>>, y: &mut i32, x: &mut i32, dir: &mut (i32, i32)) {
    let new_y = *y + dir.0;
    let new_x = *x + dir.1;

    if new_y > -1 && new_y < m.len() as i32 &&
        new_x > -1 && new_x < m[0].len() as i32 {
            if m[new_y as usize][new_x as usize] == '#' {
                turn_right(dir, &mut m[*y as usize][*x as usize]);
            } else {
                m[new_y as usize][new_x as usize] = m[*y as usize][*x as usize];
                m[*y as usize][*x as usize] = '.';
                *y = new_y;
                *x = new_x;
            }
        } else {
            *y = new_y;
            *x = new_x;
        }
}

pub fn part1(input: &str) -> i32 {
    let (mut m, pos) = load_matrix(input);
    if pos.is_none() {
        panic!("Could not find starting position")
    }
    let (mut cur_y, mut cur_x) = pos.unwrap();
    let mut cur_dir: (i32, i32) = (-1, 0);
    
    // keep a hash of visited positions and if we visit one twice, it's fine
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    while cur_y > -1 && cur_y < m.len() as i32 &&
        cur_x > -1 && cur_x < m[0].len() as i32 {
        visited.insert((cur_y, cur_x), true);
        move_or_turn(&mut m, &mut cur_y, &mut cur_x, &mut cur_dir);
    }
    visited.len() as i32
}

fn move_or_turn2(m: &Vec<Vec<char>>, y: &mut i32, x: &mut i32, dir: &mut (i32, i32)) {
    let new_y = *y + dir.0;
    let new_x = *x + dir.1;

    if new_y > -1 && new_y < m.len() as i32 &&
        new_x > -1 && new_x < m[0].len() as i32 {
        if m[new_y as usize][new_x as usize] == '#' {
            *dir = (dir.1, -dir.0);
        } else {
            *y = new_y;
            *x = new_x;
        }
    } else {
        *y = new_y;
        *x = new_x;
    }
}

fn is_loop(m: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    let (mut cur_y, mut cur_x) = pos;
    let mut cur_dir: (i32, i32) = (-1, 0);

    // this time keep a hash that includes y, x and the direction as well
    //  with a count. If a count exceeds 1 we are in a loop
    let mut visited: HashMap<(i32, i32, (i32, i32)), i8> = HashMap::new();
    let mut found_loop = false;
    while cur_y > -1 && cur_y < m.len() as i32 &&
        cur_x > -1 && cur_x < m[0].len() as i32 {
        *visited.entry((cur_y, cur_x, (cur_dir))).or_insert(0) += 1;
        if let Some(count) = visited.get(&(cur_y, cur_x, (cur_dir))) {
            if *count > 1 {
                found_loop = true;
                break;
            }
        }
        move_or_turn2(&m, &mut cur_y, &mut cur_x, &mut cur_dir);
    }
    return found_loop;
}

pub fn part2(input: &str) -> i32 {
    let (mut m, pos) = load_matrix(input);
    if pos.is_none() {
        panic!("Could not find starting position")
    }
    let (mut cur_y, mut cur_x) = pos.unwrap();
    let start_y = cur_y;
    let start_x = cur_x;
    let mut total = 0;
    
    // will use an is_loop function and then replace each '.'
    for y in 0..m.len() {
        for x in 0..m[0].len() {
            if !(y as i32 == start_y && x as i32 == start_x) && m[y][x] == '.' {
                let mut cur_m = m.clone();
                cur_m[y][x] = '#';
                if is_loop(&cur_m, (start_y, start_x)) {
                    total += 1;
                }
            }
        }
    }
                
    total
}

#[cfg(test)]
mod tests {
    use std::fs;
    use once_cell::sync::Lazy;
    use super::*;
    
    static TEST_INPUT: Lazy<String> = Lazy::new(|| {
        fs::read_to_string("aoc24_day06/test.txt")
            .expect("Could not read test file")
        });

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 6);
    }
}