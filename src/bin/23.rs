use std::collections::HashMap;
use colored::Colorize;

advent_of_code::solution!(23);

type Position = (isize, isize);

const N: Position = (0, -1);
const E: Position = (1, 0);
const S: Position = (0, 1);
const W: Position = (-1, 0);

fn get_walkable_neighbours(map: &Vec<Vec<char>>, path: &Vec<Position>, ignore_slopes: bool) -> Option<Vec<Position>> {
    let position = path[path.len() - 1];
    let mut neighbours = vec![];
    let directions = if ignore_slopes {
        vec![N, E, S, W]
    } else {
        match map[position.1 as usize][position.0 as usize] {
            '^' => vec![N],
            '>' => vec![E],
            'v' => vec![S],
            '<' => vec![W],
            '.' => vec![N, E, S, W],
            _ => unreachable!()
        }
    };

    for direction in directions {
        let next = (position.0 + direction.0, position.1 + direction.1);
        if next.0 < 0 || next.1 < 0 || next.0 >= map[0].len() as isize || next.1 >= map.len() as isize {
            continue;
        }

        if path.contains(&next) {
            continue;
        }

        let neighbour = map[next.1 as usize][next.0 as usize];
        if !ignore_slopes {
            match neighbour {
                '^' if direction != N => continue,
                '>' if direction != E => continue,
                'v' if direction != S => continue,
                '<' if direction != W => continue,
                '#' => continue,
                _ => {}
            }
        } else if neighbour == '#' {
            continue;
        }

        neighbours.push(next);
    }

    if neighbours.len() == 0 {
        return None;
    }

    Some(neighbours)
}

fn print_path(map: &Vec<Vec<char>>, path: &Vec<Position>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if path.contains(&(x as isize, y as isize)) {
                print!("{}", "O".red());
            } else {
                print!("{}", map[y][x]);
            }
        }
        println!();
    }
    println!();
}

// Update this to collapse the graph and do DFS instead of BFS
fn solve(input: &str, ignore_slopes: bool) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut paths = vec![vec![(1, 0)]];
    let mut paths_from_junctions: Vec<Vec<(isize, isize)>> = vec![vec![]];
    let mut paths_finished = false;
    while !paths_finished {
        paths_finished = true;
        for i in 0..paths.len() {
            if let Some(next) = get_walkable_neighbours(&map, &paths[i], ignore_slopes) {
                for j in 1..next.len() {
                    let mut new_path = paths[i].clone();
                    new_path.push(next[j]);
                    paths.push(new_path);
                }
                paths[i].push(next[0]);

                paths_finished = false;
            } else {

            }
        }
    }

    // Discard paths that didn't reach the exit
    paths.retain(|path| path.last().unwrap().1 == map.len() as isize - 1);
    paths.sort_by(|a, b| b.len().cmp(&a.len()));

    // print_path(&map, &paths[0]);

    Some(paths[0].len() as u32 - 1)
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
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
