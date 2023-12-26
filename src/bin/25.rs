use std::collections::{HashMap, HashSet};

advent_of_code::solution!(25);

type Graph = HashMap<String, Vec<String>>;

fn parse(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        let values: Vec<String> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        for value in &values {
            graph
                .entry(value.clone())
                .or_insert(vec![])
                .push(key.to_string());
            graph
                .entry(key.to_string())
                .or_insert(vec![])
                .push(value.clone());
        }
    }

    graph
}

fn find_left_size(graph: &Graph) -> u32 {
    let start = graph.keys().next().unwrap();
    let mut component = HashSet::from([start.to_string()]);

    let mut middle_edges = graph[start]
        .iter()
        .map(|end| (start.to_string(), end.to_string()))
        .collect::<HashSet<_>>();

    while middle_edges.len() > 3 {
        let next = middle_edges
            .iter()
            .cloned()
            .map(|(_start, end)| end)
            .min_by_key(|end| {
                graph[end]
                    .iter()
                    .filter(|node| !component.contains(*node))
                    .count()
            })
            .unwrap();

        component.insert(next.to_string());

        for neighbour in &graph[&next] {
            if component.contains(neighbour) {
                middle_edges.remove(&(neighbour.to_string(), next.to_string()));
            } else {
                middle_edges.insert((next.to_string(), neighbour.to_string()));
            }
        }
    }

    component.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = parse(input);

    let left_size = find_left_size(&graph);

    Some(left_size * (graph.len() as u32 - left_size))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
