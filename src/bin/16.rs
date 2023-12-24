use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashSet;
use std::result::Result;

advent_of_code::solution!(16);

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);
type Direction = (isize, isize);
type VisitedSet = HashSet<(Position, Direction)>;

const NORTH: Direction = (0, -1);
const SOUTH: Direction = (0, 1);
const EAST: Direction = (1, 0);
const WEST: Direction = (-1, 0);

struct Beam<'a> {
    grid: &'a Grid,
    visited: &'a RefCell<VisitedSet>,
    position: Position,
    direction: Direction,
}

impl<'a> Beam<'a> {
    fn new(
        grid: &'a Grid,
        visited: &'a RefCell<VisitedSet>,
        position: Position,
        direction: Direction,
    ) -> Self {
        Self {
            grid,
            visited,
            position,
            direction,
        }
    }

    fn get_current_tile(&self) -> char {
        self.grid[self.position.1][self.position.0]
    }

    fn can_advance(&self, direction: Direction) -> bool {
        let (px, py) = self.position;
        let (dx, dy) = direction;
        let (next_x, next_y) = ((px as isize + dx), (py as isize + dy));

        (0..self.grid[0].len()).contains(&(next_x as usize))
            && (0..self.grid.len()).contains(&(next_y as usize))
            && !self
                .visited
                .borrow()
                .contains(&(((next_x as usize), (next_y as usize)), direction))
    }

    fn advance(&mut self) {
        while self.can_advance(self.direction) {
            let (px, py) = self.position;
            let (dx, dy) = self.direction;
            let (next_x, next_y) = ((px as isize + dx), (py as isize + dy));

            self.position = ((next_x as usize), (next_y as usize));
            self.visited
                .borrow_mut()
                .insert((self.position, self.direction));

            if self.get_current_tile() != '.' {
                break;
            }
        }
    }

    fn try_split(&self, direction: Direction) -> Option<Beam<'a>> {
        if self.can_advance(direction) {
            let mut beam: Beam<'a> = Beam::new(self.grid, self.visited, self.position, direction);
            beam.advance();
            Some(beam)
        } else {
            None
        }
    }

    fn step(&mut self) -> Result<Option<Beam<'a>>, Option<Beam<'a>>> {
        let mut new_beam: Option<Beam<'a>> = None;

        let current_tile = self.get_current_tile();
        if current_tile != '.' {
            self.direction = match current_tile {
                '|' if self.direction == WEST || self.direction == EAST => {
                    new_beam = self.try_split(NORTH);
                    SOUTH
                }
                '-' if self.direction == NORTH || self.direction == SOUTH => {
                    new_beam = self.try_split(EAST);
                    WEST
                }
                '/' => match self.direction {
                    EAST => NORTH,
                    SOUTH => WEST,
                    NORTH => EAST,
                    WEST => SOUTH,
                    _ => panic!("Invalid direction!"),
                },
                '\\' => match self.direction {
                    EAST => SOUTH,
                    SOUTH => EAST,
                    NORTH => WEST,
                    WEST => NORTH,
                    _ => panic!("Invalid direction!"),
                },
                _ => self.direction,
            };
        }

        if self.can_advance(self.direction) {
            self.advance();
            return Ok(new_beam);
        }

        Err(new_beam)
    }
}

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn simulate(grid: &Grid, start_position: Position, start_direction: Direction) -> Option<u32> {
    let visited: RefCell<VisitedSet> = RefCell::new(HashSet::new());
    visited
        .borrow_mut()
        .insert((start_position, start_direction));
    let mut beams: Vec<Beam> = vec![Beam::new(&grid, &visited, start_position, start_direction)];

    loop {
        let visited_count = visited.borrow().len();
        for i in (0..beams.len()).rev() {
            match beams[i].step() {
                Ok(Some(new_beam)) => {
                    beams.push(new_beam);
                }
                Ok(None) => {}
                Err(Some(new_beam)) => {
                    beams.push(new_beam);
                    beams.remove(i);
                }
                Err(None) => {
                    beams.remove(i);
                }
            }
        }

        if visited.borrow().len() == visited_count {
            break;
        }
    }

    let mut energised: HashSet<Position> = HashSet::new();
    for (position, _) in visited.borrow().iter() {
        energised.insert(*position);
    }

    Some(energised.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    simulate(&grid, (0, 0), EAST)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let mut max_energised: u32 = 0;

    for x in 0..grid[0].len() {
        max_energised = max(simulate(&grid, (x, 0), SOUTH)?, max_energised);
        max_energised = max(simulate(&grid, (x, grid.len() - 1), NORTH)?, max_energised);
    }

    for y in 0..grid.len() {
        max_energised = max(simulate(&grid, (0, y), EAST)?, max_energised);
        max_energised = max(
            simulate(&grid, (grid[0].len() - 1, y), WEST)?,
            max_energised,
        );
    }

    Some(max_energised as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
