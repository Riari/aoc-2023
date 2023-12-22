use std::cmp::max;
use itertools::Itertools;

advent_of_code::solution!(22);

type XY = (usize, usize);
type XYZ = (usize, usize, usize);

struct Brick {
    start: XYZ,
    end: XYZ,
}

fn parse(input: &str) -> Vec<Brick> {
    let mut bricks = vec![];
    for line in input.lines() {
        let (a, b) = line.split_once("~").unwrap();
        let start: XYZ = a.split(",").map(|s| s.parse().unwrap()).collect_tuple().unwrap();
        let end: XYZ = b.split(",").map(|s| s.parse().unwrap()).collect_tuple().unwrap();
        bricks.push(Brick { start, end });
    }

    bricks.sort_by(|a, b| a.start.2.cmp(&b.start.2));
    bricks
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks = parse(input);
    let highest_brick = bricks.last().unwrap();
    let max_z = max(highest_brick.start.2, highest_brick.end.2);

    // This should probably be a vec of hashset of all the positions, not just start/end positions
    let mut layers: Vec<Vec<XY>> = vec![vec![]; max_z];
    for i in 0..bricks.len() {
        for z in bricks[i].start.2..=bricks[i].end.2 {
            layers[z - 1].push((bricks[i].start.0, bricks[i].start.1));
        }
    }

    let mut settled = false;
    while !settled {
        settled = true;
        for brick in bricks.iter_mut() {
            if brick.start.2 == 1 || brick.end.2 == 1 {
                continue;
            }

            // Update this:
            // 1) iterate over the positions in the layer
            // 2) for each position, check each component to see if it's contained by the brick's start/end range for that component
            // 3) if it is, skip this brick
            // 4) otherwise, update the brick position, add the positions it occupies to the new layer, and remove its positions from the layer it was in
            if !layers[brick.start.2 - 2].contains(&(brick.start.0, brick.start.1)) {
                brick.start.2 -= 1;
                brick.end.2 -= 1;
                settled = false;
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
