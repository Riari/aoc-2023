use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_DIGITS: Regex = Regex::new(r"\d").unwrap();
    static ref WORD_MAP: HashMap<String, u32> = [
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9)
    ].iter().cloned().collect();
}

advent_of_code::solution!(1);

fn solve(input: &str, with_words: bool) -> Option<u32> {
    let mut values: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut digits: HashMap<usize, u32> = HashMap::new();
        let mut matches = RE_DIGITS.find_iter(line);
        if let Some(first) = matches.next() {
            let last = matches.last().unwrap_or(first);
            digits.insert(first.start(), first.as_str().parse::<u32>().unwrap());
            digits.insert(last.start(), last.as_str().parse::<u32>().unwrap());
        }

        if with_words {
            let mut match_count = 0;
            for (i, chars) in line.chars().collect::<Vec<char>>().windows(5).enumerate() {
                let substring: String = chars.iter().collect();
                
                for (word, digit) in WORD_MAP.iter() {
                    if let Some(position) = substring.find(word) {
                        digits.insert(i + position, *digit);
                        match_count += 1;
                    }
                }
            }

            if match_count == 1 {
                // Only one word encountered, so it's the first and last
                let (pos, digit) = digits.iter().last().unwrap();
                digits.insert(*pos, *digit);
            }
        }

        if digits.len() == 0 {
            // No digits encountered, so skip this line
            continue;
        }

        let min = *digits.keys().min().unwrap_or(&usize::max_value());
        let max = *digits.keys().max().unwrap_or(&0);

        values.push(digits[&min] * 10 + digits[&max]);
    }

    Some(values.iter().sum())
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
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
