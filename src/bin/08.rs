use advent_of_code::lcm_of_vec;
use std::collections::HashMap;

advent_of_code::solution!(8);

fn solve(input: &str, ghost_mode: bool) -> Option<u64> {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut positions: Vec<&str> = vec![];
    if !ghost_mode {
        positions.push("AAA");
    }

    for line in lines {
        let parts = line.split(" = ").collect::<Vec<_>>();
        let start = parts[0];
        if ghost_mode && start.ends_with('A') {
            positions.push(start);
        }
        let destinations = parts[1].split(", ").collect::<Vec<_>>();
        let mut a = destinations[0].chars();
        a.next();
        let mut b = destinations[1].chars();
        b.next_back();

        map.insert(start, (a.as_str(), b.as_str()));
    }

    let mut steps: usize = 0;
    let mut loop_sizes: HashMap<u64, u64> = HashMap::new();
    loop {
        for i in 0..positions.len() {
            if loop_sizes.contains_key(&(i as u64)) {
                continue;
            }

            let current_node = map.get(positions[i]).unwrap();
            match instructions[steps % instructions.len()] {
                'L' => positions[i] = current_node.0,
                'R' => positions[i] = current_node.1,
                _ => unreachable!(),
            }

            if !ghost_mode && positions[i] == "ZZZ" || ghost_mode && positions[i].ends_with('Z') {
                loop_sizes.insert(i as u64, steps as u64 + 1);
            }
        }

        steps += 1;

        if loop_sizes.len() == positions.len() {
            break;
        }
    }

    let result = lcm_of_vec(loop_sizes.values().cloned().collect());

    Some(result as u64)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
