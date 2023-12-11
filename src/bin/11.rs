use std::collections::HashSet;
use itertools::Itertools;

advent_of_code::solution!(11);

type Position = (isize, isize);

fn solve(input: &str, expand_by: isize) -> Option<u64> {
    let mut occupied_rows: HashSet<isize> = HashSet::new();
    let mut occupied_cols: HashSet<isize> = HashSet::new();
    let mut map: Vec<Vec<char>> = vec![];
    let mut galaxies: Vec<Position> = vec![];
    for (y, line) in input.lines().enumerate() {
        map.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                occupied_rows.insert(y as isize);
                occupied_cols.insert(x as isize);
                galaxies.push((x as isize, y as isize));
            }

            map[y].push(c);
        }
    }

    let mut empty_rows: HashSet<isize> = HashSet::new();
    let mut empty_cols: HashSet<isize> = HashSet::new();
    for y in (0..map.len()).rev() {
        for x in (0..map[0].len()).rev() {
            if !occupied_cols.contains(&(x as isize)) {
                empty_cols.insert(x as isize);
            }
        }

        if !occupied_rows.contains(&(y as isize)) {
            empty_rows.insert(y as isize);
        }
    }

    let mut distance_sum = 0;
    for (from, to) in galaxies.iter().tuple_combinations() {
        let range_x = if from.0 < to.0 { from.0..to.0 } else { to.0..from.0 };
        let range_y = if from.1 < to.1 { from.1..to.1 } else { to.1..from.1 };

        for x in empty_cols.iter() {
            if range_x.contains(&x) {
                distance_sum += expand_by;
            }
        }

        for y in empty_rows.iter() {
            if range_y.contains(&y) {
                distance_sum += expand_by;
            }
        }

        distance_sum += (from.0 - to.0).abs() + (from.1 - to.1).abs();
    }

    Some(distance_sum as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 999999)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
