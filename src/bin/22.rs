use std::{ops::RangeInclusive, collections::HashSet};
use std::sync::Mutex;
use lazy_static::lazy_static;
use itertools::Itertools;

advent_of_code::solution!(22);

lazy_static! {
    static ref BRICKS: Mutex<Vec<Brick>> = Mutex::new(vec![]);

    // Indices of bricks that would cause others to fall if removed
    static ref UNSAFE_BRICK_INDICES: Mutex<HashSet<usize>> = Mutex::new(HashSet::new());
}

type Position = (usize, usize, usize);

#[derive(Clone, PartialEq, Eq)]
struct Brick {
    positions: Vec<Position>,
    layers: RangeInclusive<usize>,
}

impl Brick {
    fn move_down(&mut self) {
        for position in &mut self.positions {
            position.2 -= 1;
        }

        self.layers = self.layers.start() - 1..=self.layers.end() - 1;
    }

    fn is_settled(&self, bricks: &Vec<Brick>, ignore: Option<&Brick>) -> bool {
        if self.positions.iter().any(|p| p.2 == 1) {
            return true;
        }

        for other in bricks {
            if other == self || ignore.is_some() && other == ignore.unwrap() {
                continue;
            }

            if !self.layers.contains(&(other.layers.start() + 1))
                && !self.layers.contains(&(other.layers.end() + 1)) {
                continue;
            }

            for position in &other.positions {
                if self.positions.contains(&(position.0, position.1, position.2 + 1)) {
                    return true;
                }
            }
        }

        false
    }
}

fn parse(input: &str) {
    let mut bricks = BRICKS.lock().unwrap();

    if !bricks.is_empty() {
        return;
    }

    for line in input.lines() {
        let (a, b) = line.split_once("~").unwrap();
        let (start_x, start_y, start_z) = a.split(",").map(|s| s.parse().unwrap()).collect_tuple().unwrap();
        let (end_x, end_y, end_z) = b.split(",").map(|s| s.parse().unwrap()).collect_tuple().unwrap();
        let mut brick = Brick { positions: vec![], layers: start_z..=end_z };
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                for z in start_z..=end_z {
                    brick.positions.push((x, y, z));
                }
            }
        }

        bricks.push(brick);
    }

    bricks.sort_by(|a, b| {
        a.positions.iter()
            .map(|p| p.2)
            .min()
            .unwrap()
            .cmp(&b.positions.iter().map(|p| p.2).min().unwrap())
    });
}

fn settle(bricks: &mut Vec<Brick>) -> u32 {
    let mut fell: HashSet<usize> = HashSet::new();
    let mut settled: HashSet<usize> = HashSet::new();
    let mut is_settled = false;
    while !is_settled {
        is_settled = true;
        for i in 0..bricks.len() {
            if settled.contains(&i) {
                continue;
            }

            if bricks[i].is_settled(&bricks, None) {
                settled.insert(i);
                continue;
            }

            bricks[i].move_down();
            fell.insert(i);
            is_settled = false;
        }
    }

    fell.len() as u32
}

fn disintegrate(bricks: &mut Vec<Brick>) {
    let mut unsafe_brick_indices = UNSAFE_BRICK_INDICES.lock().unwrap();
    for i_disintegrate in 0..bricks.len() {
        for i_test in 0..bricks.len() {
            if i_disintegrate == i_test {
                continue;
            }

            if !bricks[i_test].is_settled(&bricks, Some(&bricks[i_disintegrate])) {
                unsafe_brick_indices.insert(i_disintegrate);
                break;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    parse(input);

    let mut bricks = BRICKS.lock().unwrap();

    settle(&mut bricks);
    disintegrate(&mut bricks);

    let unsafe_brick_indices = UNSAFE_BRICK_INDICES.lock().unwrap();

    Some((bricks.len() - unsafe_brick_indices.len()) as u32)
}

pub fn part_two(_: &str) -> Option<u32> {
    let bricks = BRICKS.lock().unwrap();
    let unsafe_brick_indices = UNSAFE_BRICK_INDICES.lock().unwrap();

    let mut total_fell = 0;
    for i in unsafe_brick_indices.iter() {
        let mut bricks_copy = bricks.clone();
        bricks_copy.remove(*i);
        total_fell += settle(&mut bricks_copy);
    }

    Some(total_fell)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
