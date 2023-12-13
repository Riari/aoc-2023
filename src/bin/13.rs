advent_of_code::solution!(13);

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut patterns: Vec<Vec<Vec<char>>> = vec![];
    let mut pattern: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = vec![];
            continue;
        }

        pattern.push(line.chars().collect());
    }

    if !pattern.is_empty() {
        patterns.push(pattern);
    }

    patterns
}

fn try_fold(pattern: &Vec<Vec<char>>, rotate: bool, with_smudges: bool) -> Option<u32> {
    let mut p: Vec<Vec<char>> = vec![];

    if rotate {
        // Rotate clockwise
        for i in 0..pattern[0].len() {
            let mut row: Vec<char> = pattern.iter().map(|row| row[i]).collect();
            row.reverse();
            p.push(row);
        }
    } else {
        p = pattern.clone();
    }

    for fold in 1..p.len() {
        let mut found_reflection = false;
        let mut left = fold - 1;
        let mut right = fold;
        let mut smudges = 0;
        'outer: loop {
            for i in 0..p[0].len() {
                if p[left][i] != p[right][i] {
                    if with_smudges {
                        smudges += 1;
                    } else {
                        break 'outer;
                    }
                }
            }

            if left == 0 || right == p.len() - 1 {
                if !with_smudges || smudges == 1 {
                    found_reflection = true;
                }

                break;
            }

            left -= 1;
            right += 1;
        }

        if found_reflection {
            return Some(fold as u32);
        }
    }

    None
}

fn solve(input: &str, with_smudges: bool) -> Option<u32> {
    let patterns = parse(input);
    let mut reflected_cols = 0;
    let mut reflected_rows = 0;
    for pattern in &patterns {
        if let Some(rows) = try_fold(pattern, false, with_smudges) {
            reflected_rows += rows;
        } else if let Some(cols) = try_fold(pattern, true, with_smudges) {
            reflected_cols += cols;
        }
    }

    Some((reflected_rows * 100 + reflected_cols) as u32)
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
