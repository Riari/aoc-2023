use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Clone)]
struct Card {
    winning: Vec<u32>,
    have: Vec<u32>,
    copies: u32,
}

fn parse(input: &str) -> HashMap<usize, Card> {
    let mut cards: HashMap<usize, Card> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let parts = line.split(':').collect::<Vec<&str>>();
        let numbers = parts[1].split(" | ").collect::<Vec<&str>>();
        let winning = numbers[0].split_whitespace().into_iter().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let have = numbers[1].split_whitespace().into_iter().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        cards.insert(i, Card { winning, have, copies: 1 });
    }

    cards
}

fn solve(input: &str, award_cards: bool) -> Option<u32> {
    let mut stacks = parse(input);
    let mut score = 0;
    let mut total_cards = stacks.len();
    for stack_index in 0..stacks.len() {
        let card = &stacks[&stack_index].clone();
        for _ in 0..card.copies {
            let mut winning_numbers = 0;
            let mut win_index = 0;
            for number in card.have.iter() {
                if card.winning.contains(&number) {
                    if !award_cards {
                        winning_numbers += 1;
                        continue;
                    }

                    win_index += 1;
                    stacks.get_mut(&(stack_index + win_index)).unwrap().copies += 1;
                    total_cards += 1;
                }
            }

            if !award_cards && winning_numbers > 0 {
                let mut card_score = 1;
                for _ in 1..winning_numbers {
                    card_score = card_score * 2;
                }
                score += card_score;
            }
        }
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
