advent_of_code::solution!(18);

fn solve(input: &str, p2: bool) -> Option<i64> {
    let mut vertices: Vec<(i64, i64)> = vec![(0, 0)];
    let mut perimeter: i64 = 0;
    for instruction in input.lines() {
        let mut position = vertices.last().unwrap().clone();
        let mut parts = instruction.split_whitespace();

        let direction: &str;
        let distance: i64;

        if !p2 {
            direction = parts.next().unwrap();
            distance = parts.next().unwrap().parse().unwrap();
        } else {
            parts.next();
            parts.next();
            let mut hex = parts.next().unwrap();
            hex = &hex[2..hex.len() - 1];
            distance = i64::from_str_radix(&hex[0..=4], 16).unwrap();
            direction = &hex[5..];
        }
        
        match direction {
            "U" | "3" => position.1 -= distance,
            "R" | "0" => position.0 += distance,
            "D" | "1" => position.1 += distance,
            "L" | "2" => position.0 -= distance,
            _ => unreachable!()
        }

        vertices.push(position);
        perimeter += distance;
    }

    let area = vertices
        .windows(2)
        .map(|w| {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];
            x1 * y2 - x2 * y1
        })
        .sum::<i64>().abs() / 2;

    Some(area + perimeter / 2 + 1)
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
