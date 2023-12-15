advent_of_code::solution!(15);

type Lens = (String, u32);

#[derive(Clone)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn new() -> Box {
        Box { lenses: vec![] }
    }

    fn get_lens_index(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|(l, _)| *l == label)
    }

    fn remove_lens(&mut self, label: &str) {
        if let Some(index) = self.get_lens_index(label) {
            self.lenses.remove(index);
        }
    }

    fn add_lens<'a>(&mut self, label: &str, focal_length: u32) {
        if let Some(index) = self.get_lens_index(label) {
            self.lenses[index] = (label.into(), focal_length);
        } else {
            self.lenses.push((label.into(), focal_length));
        }
    }
}

fn hash(input: &str) -> u32 {
    let mut hash = 0;
    for c in input.chars() {
        hash += c as u32;
        hash = hash * 17;
        hash = hash % 256;
    }

    hash
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .split_whitespace().next().unwrap()
        .split(',')
        .map(|step| hash(step))
        .collect::<Vec<u32>>()
        .iter()
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: Vec<Box> = vec![Box::new(); 256];
    for step in input.split(',') {
        if step.contains('-') {
            let label = step.split_whitespace().next().unwrap().strip_suffix("-").unwrap();
            boxes[hash(label) as usize].remove_lens(label);
        } else {
            let mut parts = step.split('=');
            let label = parts.next().unwrap();
            let focal_length = parts.next().unwrap().split_whitespace().next().unwrap().parse::<u32>().unwrap();
            boxes[hash(label) as usize].add_lens(label, focal_length);
        }
    }

    let mut total_focusing_power: u32 = 0;
    for i in 0..boxes.len() {
        for (j, (_, focal_length)) in boxes[i].lenses.iter().enumerate() {
            total_focusing_power += (i as u32 + 1) * (j as u32 + 1) * focal_length;
        }
    }

    Some(total_focusing_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
