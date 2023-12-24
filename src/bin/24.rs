use std::collections::HashSet;
use lazy_static::lazy_static;
use itertools::Itertools;
use z3::ast::{Ast, Int};

advent_of_code::solution!(24);

lazy_static! {
    static ref RE_NUMBERS: regex::Regex = regex::Regex::new(r"-?\d+").unwrap();
}

type Position = (f64, f64, f64);

struct Hailstone {
    position: Position,
    velocity: Position,
}

impl Hailstone {
    fn new(position: Position, velocity: Position) -> Self {
        Self {
            position,
            velocity,
        }
    }

    fn intersect(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let m1 = self.velocity.1 / self.velocity.0;
        let m2 = other.velocity.1 / other.velocity.0;
        if (m2 - m1).abs() < f64::EPSILON {
            return None;
        }

        let x = (m1 * self.position.0 - m2 * other.position.0 + other.position.1 - self.position.1) / (m1 - m2);
        let y = (m1 * m2 * (other.position.0 - self.position.0) + m2 * self.position.1 - m1 * other.position.1) / (m2 - m1);

        Some((x,y))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut hailstones: Vec<Hailstone> = vec![];
    for line in input.lines() {
        let numbers = RE_NUMBERS.find_iter(line).map(|x| x.as_str().parse().unwrap()).collect_vec();
        let position = (numbers[0], numbers[1], numbers[2]);
        let velocity = (numbers[3], numbers[4], numbers[5]);
        hailstones.push(Hailstone::new(position, velocity));
    }

    let bounds = 200000000000000.0..=400000000000000.0;
    let intersects = hailstones
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            if let Some(point) = a.intersect(b) {
                return a.velocity.0.signum() == (point.0 - a.position.0).signum()
                    && b.velocity.0.signum() == (point.0 - b.position.0).signum()
                    && bounds.contains(&point.0)
                    && bounds.contains(&point.1);
            }

            false
        })
        .count();

    Some(intersects as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
