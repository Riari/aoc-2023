use std::collections::HashMap;
use std::cmp::Ordering;
use lazy_static::lazy_static;

advent_of_code::solution!(7);

lazy_static! {
    static ref CARD_TO_STRENGTH: HashMap<char, u32> = [
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ].iter().cloned().collect();
}

type Hand = Vec<u32>;

fn parse(input: &str) -> Vec<(Hand, u32, u32)> {
    let mut card_map: HashMap<char, u32> = HashMap::new();
    let mut hands: Vec<(Hand, u32, u32)> = input.lines().map(|line| {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let mut hand = Vec::new();
        for c in parts[0].chars() {
            *card_map.entry(c).or_insert(0) += 1;
            hand.push(CARD_TO_STRENGTH[&c]);
        }

        let bid = parts[1].parse().unwrap();

        let card_type: Vec<u32> = card_map.clone().into_iter().map(|(_, count)| count).collect();

        let mut rank = 0;
        if card_type.contains(&5) {
            rank = 6;
        } else if card_type.contains(&4) {
            rank = 5;
        } else if card_type.contains(&3) && card_type.contains(&2) {
            rank = 4;
        } else if card_type.contains(&3) {
            rank = 3;
        } else if card_type.iter().filter(|count| **count == 2).count() == 2 {
            rank = 2;
        } else if card_type.contains(&2) {
            rank = 1;
        }

        card_map.clear();

        (hand, bid, rank)
    }).collect();

    hands.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    hands
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = parse(input);

    hands.sort_by(|a, b| {
        if a.2 != b.2 {
            return Ordering::Equal;
        }

        for pair in a.0.iter().zip(b.0.iter()).into_iter() {
            let ordering = pair.0.cmp(pair.1);
            if ordering == Ordering::Equal {
                continue;
            }
            
            return ordering;
        }

        Ordering::Equal
    });

    let mut winnings = 0;
    for (adjusted_rank, hand) in hands.iter().enumerate() {
        winnings += (adjusted_rank as u32 + 1) * hand.1;
    }

    Some(winnings)
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
