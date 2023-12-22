use std::collections::HashMap;
use num::Integer;

advent_of_code::solution!(21);

type Position = (isize, isize);

struct Map {
    value: Vec<Vec<char>>,
    size: isize,
}

impl Map {
    fn new(input: &str) -> Self {
        let value: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let size = value.len() as isize;
        Map { value, size }
    }

    fn get(&self, mut x: isize, mut y: isize) -> char {
        if x < 0 {
            x = self.size - x;
        }
        if y < 0 {
            y = self.size - y;
        }

        self.value[(y % self.size) as usize][(x % self.size) as usize]
    }
}

const N: Position = (0, -1);
const S: Position = (0, 1);
const E: Position = (1, 0);
const W: Position = (-1, 0);

fn get_plots_adjacent_to(x: isize, y: isize, map: &Map) -> Vec<Position> {
    let mut plots = vec![];
    for (dx, dy) in &[N, S, E, W] {
        let (nx, ny) = (x + dx, y + dy);
        if map.get(nx, ny) == '.' {
            plots.push((nx, ny));
        }
    }
    plots
}

fn solve(input: &str, is_p2: bool) -> Option<u32> {
    let map: Map = Map::new(input);
    let mut visited: HashMap<Position, u32> = HashMap::new();

    // Input is assumed to be square with starting position in the centre
    visited.insert((map.size / 2, map.size / 2), 0);

    let mut to_visit: Vec<Position> = vec![];
    for plot in get_plots_adjacent_to(map.size / 2, map.size / 2, &map) {
        to_visit.push(plot);
    }

    for steps in 1..65 {
        let mut next: Vec<Position> = vec![];
        while let Some(plot) = to_visit.pop() {
            let (x, y) = plot;
            visited.insert((x, y), steps);

            let neighbours = get_plots_adjacent_to(x, y, &map);
            for neighbour in neighbours {
                if !visited.contains_key(&neighbour) && !next.contains(&neighbour) {
                    next.push(neighbour);
                }
            }
        }
        to_visit = next;
    }

    // for y in 0..map_size {
    //     for x in 0..map_size {
    //         if visited.contains_key(&(x, y)) && visited[&(x, y)].is_even() {
    //             print!("O");
    //         } else {
    //             print!("{}", map[y as usize][x as usize]);
    //         }
    //     }
    //     println!();
    // }

    let mut reachable = 0;
    for (_, steps) in visited {
        if steps.is_even() {
            reachable += 1;
        }
    }

    Some(reachable as u32)
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
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
