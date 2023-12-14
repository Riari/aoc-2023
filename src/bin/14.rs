use std::collections::HashMap;

advent_of_code::solution!(14);

type Platform = Vec<Vec<char>>;

fn tilt(platform: &mut Platform, offset: isize, horizontally: bool) {
    let y_indices: Vec<usize> = if horizontally {
        (0..platform.len()).collect()
    } else {
        match offset {
            -1 => (1..platform.len()).collect(),
            1 => (0..platform.len() - 1).rev().collect(),
            _ => panic!("Invalid offset"),
        }
    };

    let x_indices: Vec<usize> = if horizontally {
        match offset {
            -1 => (1..platform[0].len()).collect(),
            1 => (0..platform[0].len() - 1).rev().collect(),
            _ => panic!("Invalid offset"),
        }
    } else {
        (0..platform[0].len()).collect()
    };

    loop {
        let mut changed = false;
        for y in &y_indices {
            for x in &x_indices {
                if platform[*y][*x] != 'O' {
                    continue;
                }

                if horizontally {
                    let move_to = (*x as isize + offset) as usize;

                    if platform[*y][move_to] == '.' {
                        platform[*y][*x] = '.';
                        platform[*y][move_to] = 'O';
                        changed = true;
                    }
                } else {
                    let move_to = (*y as isize + offset) as usize;
                    
                    if platform[move_to][*x] == '.' {
                        platform[*y][*x] = '.';
                        platform[move_to][*x] = 'O';
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }
}

fn solve(input: &str, do_cycles: bool) -> Option<u32> {
    let mut platform: Platform = input.lines().map(|l| l.chars().collect()).collect();
    let mut cache: HashMap<Platform, usize> = HashMap::new();
    let mut cycles = 0;
    let mut pattern_ends_at = 0;
    loop {
        tilt(&mut platform, -1, false); // North

        if !do_cycles {
            break;
        }

        tilt(&mut platform, -1, true); // West
        tilt(&mut platform, 1, false); // South
        tilt(&mut platform, 1, true); // East

        if cache.contains_key(&platform) {
            pattern_ends_at = cache[&platform];
            break;
        }

        cache.insert(platform.clone(), cycles);

        cycles += 1;
    }

    let target_cycles = 1_000_000_000;
    let mut final_cycle = 0;
    let mut load = 0;

    if !do_cycles {
        cache.insert(platform.clone(), 0);
    } else {
        final_cycle = (target_cycles - cycles - 1) % (cycles - pattern_ends_at) + pattern_ends_at;
    }

    for (value, cycle) in cache {
        if cycle != final_cycle {
            continue;
        }

        for (i, row) in value.iter().enumerate() {
            let row_load = platform.len() - i;
            load += row.iter().filter(|c| **c == 'O').count() * row_load;
        }
    }

    Some(load as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
