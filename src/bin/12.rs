use std::collections::HashMap;

advent_of_code::solution!(12);

// My solution for part 1 was very suboptimal and caused an explosion of memory usage in part 2, so I ended up
// replacing it with a conversion of this C# solution by /u/yfilipov: https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kd0u7ej/

fn count_arrangements(mut springs: &str, groups: &mut Vec<u64>, cache: &mut HashMap<String, u64>) -> u64 {
    loop {
        if groups.is_empty() {
            return if springs.contains('#') { 0 } else { 1 };
        }

        if springs.is_empty() {
            return 0;
        }

        if springs.starts_with('.') {
            springs = &mut springs.trim_start_matches('.');
            continue;
        }

        if springs.starts_with('?') {
            let functional = &mut format!(".{}", &springs[1..]);
            let damaged = &mut format!("#{}", &springs[1..]);
            return process(functional, &mut groups.clone(), cache) + process(damaged, groups, cache);
        }

        if springs.starts_with('#') {
            if groups.is_empty() {
                return 0;
            }

            if (springs.len() as u64) < groups[0] {
                return 0;
            }

            if springs[..groups[0] as usize].contains('.') {
                return 0;
            }

            if groups.len() > 1 {
                if (springs.len() as u64) < groups[0] + 1 || springs.chars().nth(groups[0] as usize).unwrap() == '#' {
                    return 0;
                }

                springs = &springs[(groups[0] + 1) as usize..];
                groups.remove(0);
                continue;
            }

            springs = &springs[groups[0] as usize..];
            groups.remove(0);
            continue;
        }

        unreachable!()
    }
}

fn process(springs: &mut String, group_sizes: &mut Vec<u64>, cache: &mut HashMap<String, u64>) -> u64 {
    let mut key = String::new();
    key.push_str(springs);
    key.extend(group_sizes.iter().map(|c| c.to_string()));

    if cache.contains_key(&key) {
        return cache[&key];
    }

    let count = count_arrangements(springs, group_sizes, cache);
    cache.insert(key, count);
    count
}

fn solve(input: &str, unfold: bool) -> Option<u64> {
    let mut cache: HashMap<String, u64> = HashMap::new();
    let mut total_arrangements = 0;
    for line in input.lines() {
        let mut parts = line.split(' ');
        let mut springs: String = parts.next().unwrap().to_owned();
        let mut group_sizes: Vec<u64> = parts.next().unwrap().split(',').map(|c| c.parse::<u64>().unwrap()).collect();

        if unfold {
            let springs_copy = springs.clone();
            let group_sizes_copy = group_sizes.clone();

            for _ in 0..4 {
                springs.push('?');
                springs.push_str(&springs_copy);
                group_sizes.extend(group_sizes_copy.iter());
            }
        }

        total_arrangements += process(&mut springs, &mut group_sizes, &mut cache);
    }

    Some(total_arrangements)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
