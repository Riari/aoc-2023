use std::collections::{HashSet, VecDeque};

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

    fn contains(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.size && y >= 0 && y < self.size
    }

    fn at(&self, x: isize, y: isize) -> Option<char> {
        if !self.contains(x, y) {
            return None;
        }

        Some(self.value[y as usize][x as usize])
    }
}

const N: Position = (0, -1);
const S: Position = (0, 1);
const E: Position = (1, 0);
const W: Position = (-1, 0);

// HyperNeutrino to the rescue again for part 2: https://www.youtube.com/watch?v=9UOMZSL0JTg&t=619s

fn walk(map: &Map, start_x: isize, start_y: isize, distance: usize) -> u64 {
    let mut accessible_plots: HashSet<Position> = HashSet::new();
    let mut seen: HashSet<Position> = HashSet::from([(start_x, start_y)]);
    let mut to_visit: VecDeque<(isize, isize, usize)> =
        VecDeque::from([(start_x, start_y, distance)]);

    while let Some((x, y, steps)) = to_visit.pop_front() {
        if steps % 2 == 0 {
            accessible_plots.insert((x, y));
        }

        if steps == 0 {
            continue;
        }

        for (dx, dy) in &[N, S, E, W] {
            let (nx, ny) = (x + dx, y + dy);
            if let Some(c) = map.at(nx, ny) {
                if c != '#' && !seen.contains(&(nx, ny)) {
                    seen.insert((nx, ny));
                    to_visit.push_back((nx, ny, steps - 1));
                }
            }
        }
    }

    accessible_plots.len() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::new(input);
    Some(walk(&map, map.size / 2, map.size / 2, 64))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::new(input);

    let start = map.size / 2;

    let steps = 26501365 as u64;
    let map_width = steps / map.size as u64 - 1;

    let odd_grids = (map_width / 2 * 2 + 1).pow(2) as u64;
    let even_grids = ((map_width + 1) / 2 * 2).pow(2) as u64;

    let odd = walk(&map, start, start, (map.size * 2 + 1) as usize);
    let even = walk(&map, start, start, (map.size * 2) as usize);

    let corner_distance = map.size as usize - 1;
    let corners = Vec::from([
        walk(&map, start, map.size - 1, corner_distance),
        walk(&map, 0, start, corner_distance),
        walk(&map, start, 0, corner_distance),
        walk(&map, map.size - 1, start, corner_distance),
    ]);

    let small_distance = map.size as usize / 2 - 1;
    let small = Vec::from([
        walk(&map, 0, map.size - 1, small_distance),
        walk(&map, map.size - 1, map.size - 1, small_distance),
        walk(&map, 0, 0, small_distance),
        walk(&map, map.size - 1, 0, small_distance),
    ]);

    let large_distance = map.size as usize * 3 / 2 - 1;
    let large = Vec::from([
        walk(&map, 0, map.size - 1, large_distance),
        walk(&map, map.size - 1, map.size - 1, large_distance),
        walk(&map, 0, 0, large_distance),
        walk(&map, map.size - 1, 0, large_distance),
    ]);

    Some(
        odd_grids * odd
            + even_grids * even
            + corners.iter().sum::<u64>()
            + (map_width + 1) * small.iter().sum::<u64>()
            + map_width * large.iter().sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(470149860542205));
    }
}
