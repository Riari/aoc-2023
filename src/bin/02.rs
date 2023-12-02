use std::collections::HashMap;

advent_of_code::solution!(2);

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn solve(input: &str, powers: bool) -> Option<u32> {
    let mut possible_games: Vec<u32> = Vec::new();
    let mut game_minimums: Vec<(u32, u32, u32)> = vec![];

    for line in input.lines() {
        let mut parts = line.split(':').into_iter();
        let game_id = parts.next().unwrap()[5..].parse::<u32>().unwrap();

        let mut is_valid = true;
        let mut counts: HashMap<String, u32> = [
            ("red".to_string(), if powers { 0 } else { MAX_RED }),
            ("green".to_string(), if powers { 0 } else { MAX_GREEN }),
            ("blue".to_string(), if powers { 0 } else { MAX_BLUE }),
        ].iter().cloned().collect();

        parts.next().unwrap().split(';').for_each(|list| {
            list.split(',').for_each(|cube| {
                let parts = cube.split_whitespace().collect::<Vec<_>>();
                let count = parts[0].parse::<u32>().unwrap();
                let colour = parts[1];
                if counts.get(colour).unwrap() < &count {
                    if powers {
                        *counts.get_mut(colour).unwrap() = count;
                    } else {
                        is_valid = false;
                    }
                }
            });
        });

        if powers {
            game_minimums.push((counts["red"], counts["green"], counts["blue"]));
            continue;
        }

        if is_valid {
            possible_games.push(game_id);
        }
    }

    if powers {
        return Some(game_minimums.iter().map(|(r, g, b)| r * g * b).sum());
    }

    Some(possible_games.iter().sum())
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
