advent_of_code::solution!(6);

fn extract_values(line: &str, concat_values: bool) -> Vec<u64> {
    let values = line.split(':').nth(1).unwrap().split_whitespace();

    if concat_values {
        let thing = values.fold(String::new(), |a, b| a + b);
        return vec![thing.parse::<u64>().unwrap()];
    }

    values
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn solve(input: &str, concat_values: bool) -> Option<u64> {
    let mut lines = input.lines();
    let times: Vec<u64> = extract_values(lines.next().unwrap(), concat_values);
    let distances: Vec<u64> = extract_values(lines.next().unwrap(), concat_values);

    let mut records_list: Vec<Vec<u64>> = vec![];
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];

        let mut records: Vec<u64> = vec![];
        let mut prev_record = 0;
        for ms in 1..time {
            let record = ms * (time - ms);
            if record > distance {
                records.push(record);
            } else if record < prev_record {
                break;
            }

            prev_record = record;
        }

        records_list.push(records);
    }

    Some(
        records_list
            .iter()
            .map(|records| records.len() as u64)
            .product(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
