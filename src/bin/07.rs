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
    static ref JOKER_VALUE: u32 = 1;
}

struct Hand {
    cards: Vec<u32>,
    score: u32,
    bid: u32
}

fn solve(input: &str, j_as_joker: bool) -> Option<u32> {
    let mut card_map: HashMap<u32, u32> = HashMap::new();
    let mut set: Vec<Hand> = input.lines().map(|line| {
        card_map.clear();

        let parts = line.split_whitespace().collect::<Vec<_>>();
        let mut cards: Vec<u32> = Vec::new();
        let mut joker_count = 0;
        for c in parts[0].chars() {
            let mut strength: u32 = CARD_TO_STRENGTH[&c];
            if j_as_joker && c == 'J' {
                strength = *JOKER_VALUE;
                joker_count += 1;
            } else {
                *card_map.entry(strength).or_insert(0) += 1;
            }

            cards.push(strength);
        }

        let bid = parts[1].parse().unwrap();

        let mut occurrences: Vec<(u32, u32)> = card_map.clone().into_iter()
            .map(|(card, count)| (card, count)).collect();

        occurrences.sort_by(|a, b| {
            let order_count = a.1.cmp(&b.1);
            if order_count == Ordering::Equal {
                return a.0.cmp(&b.0);
            }

            order_count
        });
        occurrences.reverse();

        if j_as_joker {
            if joker_count == 5 {
                return Hand { cards, score: 15, bid };
            }

            occurrences[0].1 += joker_count;
        }

        occurrences = occurrences.into_iter().filter(|(_, count)| *count > 1).collect();    

        let mut score = 0;
        if occurrences.len() > 0 {
            let g1 = occurrences[0].1;
            let g2 = if occurrences.len() > 1 { occurrences[1].1 } else { 0 };
            score = 3 * g1 + g2;
        }

        Hand { cards, score, bid }
    }).collect();

    set.sort_by(|a, b| {
        let score_order = a.score.cmp(&b.score);
        if score_order != Ordering::Equal {
            return score_order;
        }

        for (a, b) in a.cards.iter().zip(b.cards.iter()).into_iter() {
            let ordering = a.cmp(b);
            if ordering == Ordering::Equal {
                continue;
            }

            return ordering;
        }

        Ordering::Equal
    });

    let mut winnings = 0;
    for (rank, hand) in set.iter().enumerate() {
        winnings += (rank as u32 + 1) * hand.bid;
    }

    Some(winnings)
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
        assert_eq!(result, Some(6592));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6839));
    }
}
