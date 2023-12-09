advent_of_code::solution!(9);

fn solve(input: &str, get_prev: bool) -> Option<i64> {
    let mut sequence_values: Vec<i64> = vec![];
    for line in input.lines() {
        let mut stack = vec![line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()];
        while stack.last().unwrap().iter().any(|v| *v != 0) {
            let mut diffs: Vec<i64> = vec![];
            for value in stack.last().unwrap().windows(2) {
                diffs.push(value[1] - value[0]);
            }
            stack.push(diffs);
        }

        let stack_size = stack.clone().len();
        stack[stack_size - 1].push(0);

        let mut i = stack_size - 1;
        if get_prev {
            while i > 0 {
                i -= 1;
                let prev = stack[i][0];
                let sequence = prev - stack[i + 1][0];
                stack[i].insert(0, sequence);
            }
            sequence_values.push(stack[0][0]);
        } else {
            while i > 0 {
                i -= 1;
                let prev = stack[i][stack[i].len() - 1];
                let next = stack[i + 1][stack[i + 1].len() - 1] + prev;
                stack[i].push(next);
            }
            sequence_values.push(stack[0][stack[0].len() - 1]);
        }

    }

    Some(sequence_values.iter().sum())
}

pub fn part_one(input: &str) -> Option<i64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
