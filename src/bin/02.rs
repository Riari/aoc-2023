use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(2);

lazy_static! {
    static ref RE_GAME: Regex = Regex::new(r"Game (\d+):\s*((?:\d+\s\w+[,;]?\s*)+)").unwrap();
    static ref RE_SET: Regex = Regex::new(r"(\d+)\s(\w+)").unwrap();
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn solve(input: &str, powers: bool) -> Option<u32> {
    let mut possible_games: Vec<u32> = Vec::new();
    let mut game_minimums: Vec<(u32, u32, u32)> = vec![];

    for line in input.lines() {
        let game = RE_GAME.captures(line)?;
        let game_id = game[1].parse::<u32>().unwrap();

        let mut is_valid = true;
        let mut counts = (0, 0, 0);

        game[2].split(';').for_each(|list| {
            list.split(',').for_each(|cube| {
                let caps = RE_SET.captures(cube).unwrap();
                let count = caps[1].parse::<u32>().unwrap();
                match caps[2].trim() {
                    "red" => {
                        if powers && count > counts.0 { counts.0 = count; }
                        else if count > MAX_RED { is_valid = false; }
                    },
                    "green" => {
                        if powers && count > counts.1 { counts.1 = count; }
                        else if count > MAX_GREEN { is_valid = false; }
                    },
                    "blue" => {
                        if powers && count > counts.2 { counts.2 = count; }
                        else if count > MAX_BLUE { is_valid = false; }
                    },
                    _ => panic!("Unknown colour!")
                }
            });
        });

        if powers {
            game_minimums.push(counts);
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
