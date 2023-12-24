use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

type Position = (isize, isize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Connection {
    to: Position,
    distance: usize,
    accessible: bool,
}

const N: Position = (0, -1);
const E: Position = (1, 0);
const S: Position = (0, 1);
const W: Position = (-1, 0);

fn find_connection(map: &Vec<Vec<char>>, path: &mut Vec<Position>) -> Option<(usize, Position)> {
    let mut position = *path.last().unwrap();

    let neighbours = get_walkable_neighbours(map, path, false);
    if neighbours.is_none() {
        return None;
    }

    let mut next = neighbours.unwrap();
    while next.len() == 1 {
        position = next[0].0;
        path.push(position.clone());
        if let Some(neighbours) = get_walkable_neighbours(map, path, false) {
            next = neighbours;
        } else {
            break;
        }
    }

    Some((path.len(), position.clone()))
}

fn contract_map(map: &Vec<Vec<char>>) -> HashMap<Position, Vec<Connection>> {
    let mut nodes: HashMap<Position, Vec<Connection>> = HashMap::new();

    // Insert start and end nodes, even though they're not junctions
    nodes.insert((1, 0), vec![]);
    nodes.insert((map[0].len() as isize - 2, map.len() as isize - 1), vec![]);

    // Find the junction nodes
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != '.' {
                continue;
            }

            if let Some(neighbours) =
                get_walkable_neighbours(map, &vec![(x as isize, y as isize)], true)
            {
                if neighbours.len() == 1 {
                    continue;
                }

                nodes.insert((x as isize, y as isize), vec![]);
            }
        }
    }

    // Find the edges between the nodes
    for (node, connections) in nodes.iter_mut() {
        let neighbours = get_walkable_neighbours(map, &vec![*node], false).unwrap();
        for (neighbour, accessible) in neighbours {
            if let Some(connection) = find_connection(map, &mut vec![*node, neighbour]) {
                connections.push(Connection {
                    to: connection.1,
                    distance: connection.0 - 1,
                    accessible,
                });
            }
        }
    }

    nodes
}

fn get_walkable_neighbours(
    map: &Vec<Vec<char>>,
    path: &Vec<Position>,
    slopes_only: bool,
) -> Option<Vec<(Position, bool)>> {
    let position = path[path.len() - 1];
    let mut neighbours = vec![];

    for direction in vec![N, E, S, W] {
        let next = (position.0 + direction.0, position.1 + direction.1);
        if next.0 < 0
            || next.1 < 0
            || next.0 >= map[0].len() as isize
            || next.1 >= map.len() as isize
        {
            continue;
        }

        if path.contains(&next) {
            continue;
        }

        let neighbour = map[next.1 as usize][next.0 as usize];

        if neighbour == '#' || slopes_only && neighbour == '.' {
            continue;
        }

        let accessible = match neighbour {
            '^' if direction != N => false,
            '>' if direction != E => false,
            'v' if direction != S => false,
            '<' if direction != W => false,
            _ => true,
        };

        neighbours.push((next, accessible));
    }

    if neighbours.len() == 0 {
        return None;
    }

    Some(neighbours)
}

fn find_path_lengths(
    graph: &HashMap<Position, Vec<Connection>>,
    start: Position,
    end: Position,
    ignore_slopes: bool,
) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut current_path = vec![];
    let mut lengths = vec![];

    dfs(
        &graph,
        start,
        end,
        &mut visited,
        &mut current_path,
        &mut lengths,
        0,
        ignore_slopes,
    );

    lengths
}

fn dfs(
    graph: &HashMap<Position, Vec<Connection>>,
    current: Position,
    end: Position,
    visited: &mut HashSet<Position>,
    current_path: &mut Vec<Position>,
    path_lengths: &mut Vec<usize>,
    current_distance: usize,
    ignore_slopes: bool,
) {
    visited.insert(current);
    current_path.push(current);

    if current == end {
        path_lengths.push(current_distance);
        visited.remove(&current);
        current_path.pop();
        return;
    }

    if let Some(connections) = graph.get(&current) {
        for connection in connections {
            if visited.contains(&connection.to) || (!ignore_slopes && !connection.accessible) {
                continue;
            }

            let new_distance = current_distance + connection.distance;
            dfs(
                graph,
                connection.to,
                end,
                visited,
                current_path,
                path_lengths,
                new_distance,
                ignore_slopes,
            );
        }
    }

    visited.remove(&current);
    current_path.pop();
}

fn solve(input: &str, ignore_slopes: bool) -> Option<u32> {
    let map = &input.lines().map(|line| line.chars().collect()).collect();
    let nodes = contract_map(map);

    let lengths = find_path_lengths(
        &nodes,
        (1, 0),
        (map[0].len() as isize - 2, map.len() as isize - 1),
        ignore_slopes,
    );

    Some(*lengths.iter().max().unwrap() as u32)
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
