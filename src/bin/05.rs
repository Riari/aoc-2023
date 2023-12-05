use std::ops::Range;

advent_of_code::solution!(5);

#[derive(Clone)]
struct Map {
    source_ranges: Vec<Range<u64>>,
    target_ranges: Vec<Range<u64>>,
}

impl Map {
    fn new() -> Self {
        Map { source_ranges: Vec::new(), target_ranges: Vec::new() }
    }

    fn clear(&mut self) {
        self.source_ranges.clear();
        self.target_ranges.clear();
    }
}

fn parse(input: &str, seeds_as_ranges: bool) -> (Vec<Range<u64>>, Vec<Map>) {
    let mut lines = input.lines();
    let seeds: Vec<u64> = lines.next().unwrap().split(": ").nth(1).unwrap().split(' ').map(|value| value.parse::<u64>().unwrap()).collect();
    let mut maps: Vec<Map> = Vec::new();
    let mut map = Map::new();

    lines.next(); // skip the first blank line
    for line in lines {
        if line.is_empty() {
            // Finish the maps
            maps.push(map.clone());
            map.clear();
            continue;
        }

        if line.ends_with("map:") {
            continue;
        }

        // If we get this far, the line should contain map values
        let values: Vec<u64> = line.split(' ').map(|value| value.parse::<u64>().unwrap()).collect();
        map.source_ranges.push(values[1]..values[1] + values[2]);
        map.target_ranges.push(values[0]..values[0] + values[2]);
    }

    // Make sure to push the very last map
    maps.push(map);

    let mut seed_ranges: Vec<Range<u64>> = vec![];
    if seeds_as_ranges {
        // Interpret the seed values as pairs of range start and range length
        for i in (0..seeds.len() - 1).step_by(2) {
            let start = seeds[i];
            let length = seeds[i + 1];
            seed_ranges.push(start..start + length);
        }

        return (seed_ranges, maps);
    }

    for seed in seeds {
        seed_ranges.push(seed..seed + 1);
    }

    (seed_ranges, maps)
}

fn solve(input: &str, as_ranges: bool) -> Option<u64> {
    let (seeds, maps) = parse(input, as_ranges);
    let mut locations: Vec<u64> = vec![];
    
    for seed_range in seeds {
        for seed in seed_range {
            let mut value = seed;
            for map in maps.iter() {
                for (range_index, source_range) in map.source_ranges.iter().enumerate() {
                    if source_range.contains(&value) {
                        value = map.target_ranges[range_index].clone().nth((value - source_range.start) as usize).unwrap();
                        break;
                    }
                }
            }
    
            locations.push(value);
        }
    }

    Some(*locations.iter().min().unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
