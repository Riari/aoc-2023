use std::collections::HashMap;
use num::Integer;

advent_of_code::solution!(21);

type Position = (isize, isize);

const N: Position = (0, -1);
const S: Position = (0, 1);
const E: Position = (1, 0);
const W: Position = (-1, 0);

fn is_valid_position(x: isize, y: isize, map_size: isize) -> bool {
    x >= 0 && y >= 0 && x < map_size && y < map_size
}

fn get_plots_adjacent_to(x: isize, y: isize, map: &Vec<Vec<char>>) -> Vec<Position> {
    let mut plots = vec![];
    for (dx, dy) in &[N, S, E, W] {
        let (nx, ny) = (x + dx, y + dy);
        if !is_valid_position(nx, ny, map.len() as isize) {
            continue;
        }
        if map[ny as usize][nx as usize] == '.' {
            plots.push((nx, ny));
        }
    }
    plots
}

fn solve(input: &str, is_p2: bool) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited: HashMap<Position, u32> = HashMap::new();

    // Input is assumed to be square with starting position in the centre
    let map_size = map.len() as isize;
    visited.insert((map_size / 2, map_size / 2), 0);

    let mut to_visit: Vec<Position> = vec![];
    for plot in get_plots_adjacent_to(map_size / 2, map_size / 2, &map) {
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
