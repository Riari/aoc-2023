use lazy_static::lazy_static;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

advent_of_code::solution!(19);

lazy_static! {
    static ref RE_NUMBERS: regex::Regex = regex::Regex::new(r"(\d+)").unwrap();
    static ref ACCEPTED: String = "A".to_string();
    static ref REJECTED: String = "R".to_string();
    static ref PROCESSED: Vec<String> = vec![ACCEPTED.clone(), REJECTED.clone()];
}

#[derive(Clone, Eq, PartialEq)]
enum Result {
    Accept,
    Reject,
    Destination,
}

impl Result {
    fn from_str(s: &str) -> Result {
        match s {
            "A" => Result::Accept,
            "R" => Result::Reject,
            _ => Result::Destination,
        }
    }

    fn to_string(&self, destination: Option<String>) -> String {
        match self {
            Result::Accept => ACCEPTED.clone(),
            Result::Reject => REJECTED.clone(),
            Result::Destination => destination.unwrap(),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Clone)]
struct Rule {
    category: usize, // index into part (x,m,a,s)
    target_rating: u64,
    operator: Operator,
}

#[derive(Clone)]
struct Step {
    rule: Option<Rule>,
    result: Result,
    destination: Option<String>,
}

struct Workflow {
    steps: Vec<Step>,
}

impl Workflow {
    fn accepts(&self, ratings: &Vec<u64>) -> &Step {
        for step in self.steps.iter() {
            if let Some(rule) = &step.rule {
                if !(match rule.operator {
                    Operator::LessThan => ratings[rule.category] < rule.target_rating,
                    Operator::GreaterThan => ratings[rule.category] > rule.target_rating,
                }) {
                    continue;
                }
            }

            return step;
        }

        unreachable!()
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Part {
    status: String, // indicates a workflow or ACCEPTED/REJECTED
    ratings: Vec<u64>,
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = vec![];
    let mut is_parsing_workflows = true;
    for line in input.lines() {
        if line.is_empty() {
            is_parsing_workflows = false;
            continue;
        }

        if is_parsing_workflows {
            let mut split = line.split('{');
            let id = split.next().unwrap();
            let steps_str = split.next().unwrap();
            let mut steps: Vec<Step> = vec![];
            for step_str in steps_str[..steps_str.len() - 1].split(',') {
                if step_str.contains(':') {
                    let mut rule_split = step_str.split(':');
                    let target_op_rating = rule_split.next().unwrap();
                    let result_str = rule_split.next().unwrap();
                    let category = match target_op_rating.chars().nth(0).unwrap() {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => panic!("Invalid category"),
                    };
                    let operator = match target_op_rating.chars().nth(1).unwrap() {
                        '<' => Operator::LessThan,
                        '>' => Operator::GreaterThan,
                        _ => panic!("Invalid operator"),
                    };
                    let target_rating = RE_NUMBERS
                        .find(target_op_rating)
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap();
                    let result = Result::from_str(result_str);
                    steps.push(Step {
                        rule: Some(Rule {
                            category,
                            target_rating,
                            operator,
                        }),
                        destination: if result == Result::Destination {
                            Some(result_str.to_string())
                        } else {
                            None
                        },
                        result,
                    });
                } else {
                    let result = Result::from_str(step_str);
                    steps.push(Step {
                        rule: None,
                        result: result.clone(),
                        destination: if result == Result::Destination {
                            Some(step_str.to_string())
                        } else {
                            None
                        },
                    });
                }
            }

            workflows.insert(id.to_string(), Workflow { steps });

            continue;
        }

        let ratings: Vec<u64> = RE_NUMBERS
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        parts.push(Part {
            status: "in".to_string(),
            ratings,
        });
    }

    (workflows, parts)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, mut parts) = parse(input);

    while parts
        .iter()
        .filter(|p| !PROCESSED.contains(&p.status))
        .count()
        > 0
    {
        for i in (0..parts.len()).rev() {
            if PROCESSED.contains(&parts[i].status) {
                continue;
            }

            let step = workflows
                .get(&parts[i].status)
                .unwrap()
                .accepts(&parts[i].ratings);
            parts[i].status = match step.result {
                Result::Accept => ACCEPTED.to_string(),
                Result::Reject => REJECTED.to_string(),
                Result::Destination => step.destination.clone().unwrap(),
            };
        }
    }

    let mut sum = 0;
    for part in parts {
        if part.status == *ACCEPTED {
            for rating in part.ratings {
                sum += rating;
            }
        }
    }

    Some(sum)
}

// Heavily based on HyperNeutrino's solution: https://www.youtube.com/watch?v=3RwIpUegdU4
fn count_accepted_combos(
    ranges: &Vec<(u64, u64)>,
    status: String,
    workflows: &HashMap<String, Workflow>,
) -> u64 {
    if status == *REJECTED {
        return 0;
    }

    if status == *ACCEPTED {
        let mut combos = 1;
        for range in ranges {
            combos *= range.1 - range.0 + 1;
        }

        return combos;
    }

    let workflow = workflows.get(&status).unwrap();
    let mut combos = 0;
    let mut ranges_clone = ranges.clone();
    let mut leftover_ranges = true;
    for i in 0..&workflow.steps.len() - 1 {
        let step = &workflow.steps[i];
        let rule = step.rule.as_ref().unwrap();

        let (start, end) = ranges_clone[rule.category];

        let valid_range: (u64, u64);
        let invalid_range: (u64, u64);
        if rule.operator == Operator::LessThan {
            valid_range = (start, min(rule.target_rating - 1, end));
            invalid_range = (max(rule.target_rating, start), end);
        } else {
            valid_range = (max(rule.target_rating + 1, start), end);
            invalid_range = (start, min(rule.target_rating, end));
        }

        if valid_range.0 <= valid_range.1 {
            ranges_clone[rule.category] = valid_range;
            combos += count_accepted_combos(
                &ranges_clone,
                step.result.to_string(step.destination.clone()),
                workflows,
            );
        }

        if invalid_range.0 <= invalid_range.1 {
            ranges_clone[rule.category] = invalid_range;
        } else {
            leftover_ranges = false;
            break;
        }
    }

    if leftover_ranges {
        let fallback = &workflow.steps[&workflow.steps.len() - 1];
        combos += count_accepted_combos(
            &ranges_clone,
            fallback.result.to_string(fallback.destination.clone()),
            workflows,
        );
    }

    combos
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = parse(input);
    let ranges = vec![(1 as u64, 4000 as u64); 4];
    Some(count_accepted_combos(&ranges, "in".to_string(), &workflows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
