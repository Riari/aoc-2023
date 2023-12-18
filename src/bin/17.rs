use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(17);

type Grid = Vec<Vec<u32>>;
type Position = (usize, usize);

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    position: Position,
    entered_from: Direction,
    straight_steps: u32,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Step {
    state: State,
    cost: u32,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| other.state.position.cmp(&self.state.position))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Step {
    fn new(state: State, cost: u32) -> Self {
        Self {
            state,
            cost,
        }
    }
}

impl State {
    fn new(position: Position, entered_from: Direction, straight_steps: u32) -> Self {
        Self {
            position,
            entered_from,
            straight_steps,
        }
    }

    fn can_go(&self, direction: &Direction, grid: &Grid) -> bool {
        let modifier = direction.get_offset();
        let (x, y) = (self.position.0 as isize + modifier.0, self.position.1 as isize + modifier.1);
        x >= 0 && y >= 0 && x < grid[0].len() as isize && y < grid.len() as isize
    }
}

// Implementation of Dijkstra's algorithm that considers steps taken in a direction as well as path cost.
fn solve(input: &str, min_straight_steps: u32, max_straight_steps: u32) -> Option<u32> {
    let grid: Grid = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    let start = (0, 0);
    let end = (grid[0].len() - 1, grid.len() - 1);

    let mut costs: HashMap<State, u32> = HashMap::new();
    let mut heap: BinaryHeap<Step> = BinaryHeap::new();

    let directions = vec![Direction::North, Direction::East, Direction::South, Direction::West];

    costs.insert(State { position: start, entered_from: Direction::East, straight_steps: 0 }, 0);
    costs.insert(State { position: start, entered_from: Direction::South, straight_steps: 0 }, 0);
    heap.push(Step::new(State::new(start, Direction::East, 0), 0));

    while let Some(Step { state, cost }) = heap.pop() {
        if state.position == end && state.straight_steps >= min_straight_steps {
            return Some(cost);
        }

        if costs.contains_key(&state) && costs[&state] > cost {
            continue;
        }

        for direction in directions.iter() {
            if *direction == state.entered_from.opposite() {
                continue;
            }

            if !state.can_go(direction, &grid) {
                continue;
            }

            if state.straight_steps == max_straight_steps && *direction == state.entered_from {
                continue;
            }

            let (offset_x, offset_y) = direction.get_offset();
            let new_position = ((state.position.0 as isize + offset_x) as usize, (state.position.1 as isize + offset_y) as usize);
            let neighbour = Step::new(
                State::new(
                    new_position,
                    direction.clone(),
                    if *direction == state.entered_from { state.straight_steps + 1 } else { 1 },
                ),
                cost + grid[new_position.1][new_position.0],
            );

            if (*direction == state.entered_from || state.straight_steps >= min_straight_steps)
                && neighbour.cost < *costs.get(&neighbour.state).unwrap_or(&u32::MAX) {
                heap.push(neighbour.clone());
                costs.insert(neighbour.state, neighbour.cost);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 0, 3)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
