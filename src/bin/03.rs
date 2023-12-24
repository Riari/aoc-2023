use std::collections::HashMap;

advent_of_code::solution!(3);

fn test_adjacency(
    mut start_x: usize,
    end_x: usize,
    mut start_y: usize,
    lines: Vec<&str>,
    only_gears: bool,
) -> Option<(usize, usize)> {
    let mut end_y = start_y + 1;
    if start_x > 0 {
        start_x = start_x - 1
    };
    if start_y > 0 {
        start_y = start_y - 1;
        end_y = start_y + 2;
    };

    for y in start_y..=end_y {
        if y == lines.len() {
            break;
        }

        for x in start_x..=end_x {
            if x == lines[y].len() {
                break;
            }

            let char = &lines[y].chars().nth(x).unwrap();

            if char != &'.' && !char.is_numeric() {
                if only_gears && char != &'*' {
                    continue;
                }

                return Some((x, y));
            }
        }
    }

    None
}

fn solve(input: &str, get_gear_ratios: bool) -> Option<u32> {
    let mut part_numbers: Vec<u32> = vec![];
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let lines = input.lines().collect::<Vec<_>>();

    for (y, line) in lines.iter().enumerate() {
        let chars = line.chars();
        let length = line.len();

        let mut number: Vec<u32> = vec![];

        for (x, char) in chars.enumerate() {
            let digit = char.to_digit(10);

            if digit.is_some() {
                number.push(digit.unwrap());
            }

            if (digit.is_none() || x == length - 1) && number.len() > 0 {
                if let Some(position) =
                    test_adjacency(x - number.len(), x, y, lines.clone(), get_gear_ratios)
                {
                    let value = number.iter().fold(0, |acc, elem| acc * 10 + elem);

                    if get_gear_ratios {
                        gears.entry(position).or_insert(vec![]).push(value);
                    } else {
                        part_numbers.push(value);
                    }
                }

                number = vec![];
            }
        }
    }

    if get_gear_ratios {
        return Some(
            gears
                .iter()
                .filter(|entry| entry.1.len() == 2)
                .map(|entry| entry.1.iter().product::<u32>())
                .sum(),
        );
    }

    Some(part_numbers.iter().sum())
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
        assert_eq!(result, Some(925));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6756));
    }
}
