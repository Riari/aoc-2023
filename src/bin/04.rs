use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

advent_of_code::solution!(4);

lazy_static! {
    static ref STACK: Mutex<HashMap<usize, (u32, u32)>> = Mutex::new(HashMap::new());
}

fn parse(input: &str) {
    let mut stack = STACK.lock().unwrap();
    if stack.len() > 0 {
        return;
    }

    for (i, line) in input.lines().enumerate() {
        let numbers = line.split(':').collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let winning = numbers[0].split_whitespace().map(|n| n.parse::<u32>().unwrap());
        let have = numbers[1]  .split_whitespace().map(|n| n.parse::<u32>().unwrap());
        let matching_count = winning.filter(|n| have.clone().any(|m| m == *n)).count() as u32;

        // index => number of matching numbers, number of copies
        stack.insert(i, (matching_count, 1));
    }
}

fn solve(input: &str, award_cards: bool) -> Option<u32> {
    parse(input);
    let mut stack = STACK.lock().unwrap();
    let mut score = 0;
    let mut total_cards = stack.len();
    for stack_index in 0..stack.len() {
        let card = &stack[&stack_index].clone();

        if card.0 == 0 {
            continue;
        }

        if award_cards {
            for win_index in 0..card.0 {
                total_cards += card.1 as usize;
                stack.get_mut(&(stack_index + win_index as usize + 1)).unwrap().1 += card.1;
            }
            continue;
        }
        
        score += 2u32.pow(card.0 - 1);
    }

    if award_cards {
        return Some(total_cards as u32);
    }

    Some(score)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
