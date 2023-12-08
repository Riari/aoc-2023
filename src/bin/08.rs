use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        let parts = line.split(" = ").collect::<Vec<_>>();
        let start = parts[0];
        let destinations = parts[1].split(", ").collect::<Vec<_>>();
        let mut a = destinations[0].chars();
        a.next();
        let mut b = destinations[1].chars();
        b.next_back();

        map.insert(start, (a.as_str(), b.as_str()));
    }

    let mut current_node_label = "AAA";
    let mut i = 0;
    while current_node_label != "ZZZ" {
        let current_node = map.get(current_node_label).unwrap();
        match instructions[i % instructions.len()] {
            'L' => current_node_label = current_node.0,
            'R' => current_node_label = current_node.1,
            _ => unreachable!()
        }

        i += 1;
    }
    
    Some(i as u32)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
