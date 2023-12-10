use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

advent_of_code::solution!(10);

const N: usize = 0;
const E: usize = 1;
const S: usize = 2;
const W: usize = 3;

lazy_static! {
    static ref NESW: Vec<(isize, isize)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    /// Map of pipe symbols to indices into possible entry and exit directions.
    static ref PIPE_MAP: HashMap<char, HashMap<(isize, isize), (isize, isize)>> = HashMap::from([
        ('|', HashMap::from([(NESW[N], NESW[N]), (NESW[S], NESW[S])])),
        ('-', HashMap::from([(NESW[E], NESW[E]), (NESW[W], NESW[W])])),
        ('F', HashMap::from([(NESW[N], NESW[E]), (NESW[W], NESW[S])])),
        ('7', HashMap::from([(NESW[E], NESW[S]), (NESW[N], NESW[W])])),
        ('J', HashMap::from([(NESW[S], NESW[W]), (NESW[E], NESW[N])])),
        ('L', HashMap::from([(NESW[W], NESW[N]), (NESW[S], NESW[E])])),
    ]);

    static ref CORNERS: Vec<char> = vec!['F', '7', 'J', 'L'];
}

fn apply_direction(coords: (usize, usize), direction: (isize, isize)) -> (usize, usize) {
    (
        (coords.0 as isize + direction.0) as usize,
        (coords.1 as isize + direction.1) as usize
    )
}

fn solve(input: &str, calculate_area: bool) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut coords: (usize, usize) = (0, 0);

    // Find S
    'outer: for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                coords = (x, y);
                break 'outer;
            }
        }
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(coords);

    let mut vertices: Vec<(usize, usize)> = vec![];
    vertices.push(coords);

    let mut next_direction: (isize, isize) = (0, 0);
    for direction in NESW.iter() {
        let destination_x = coords.0 as isize + direction.0;
        let destination_y = coords.1 as isize + direction.1;
        if destination_x < 0 || destination_y < 0 {
            // Out of bounds
            continue;
        }

        if let Some(pipe) = PIPE_MAP.get(&map[destination_y as usize][destination_x as usize]) {
            if let Some(_) = pipe.get(direction) {
                // Pipe can be entered from this direction
                next_direction = *direction;
                break;
            }
        }
    }

    let mut distance = 1;
    loop {
        coords = apply_direction(coords, next_direction);
        if visited.contains(&coords) {
            // Circuit completed
            break;
        }

        let pipe = &map[coords.1][coords.0];

        if CORNERS.contains(pipe) {
            vertices.insert(0, coords);
        }

        visited.insert(coords);

        next_direction = PIPE_MAP.get(pipe).unwrap().get(&next_direction).unwrap().clone();
        distance += 1;
    }

    if !calculate_area {
        return Some(distance / 2);
    }

    let mut sum: i32 = 0;
    let n = vertices.len();

    for i in 0..n {
        let j = (i + 1) % n;
        sum += (vertices[i].0 * vertices[j].1) as i32;
        sum -= (vertices[i].1 * vertices[j].0) as i32;
    }

    // Exclude the edges
    sum -= visited.len() as i32 - 2;
    let area = (sum / 2) as u32;

    Some(area)
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
        assert_eq!(result, Some(70));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
