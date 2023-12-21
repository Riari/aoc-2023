use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

const HI: u8 = 1;
const LO: u8 = 0;

type ModuleMap = HashMap<String, Box<dyn Module>>;
type ConnectionMap = HashMap<String, Vec<String>>;

trait Module {
    fn enable_input(&mut self, module: &String);
    fn receive(&mut self, module: &String, input: u8) -> bool;
    fn get_output(&mut self) -> u8;
}

struct FlipFlop {
    state: u8
}

impl Module for FlipFlop {
    fn enable_input(&mut self, _: &String) {
        // noop
    }

    fn receive(&mut self, _: &String, input: u8) -> bool {
        if input == HI {
            return false;
        }
        
        self.state = if self.state == LO { HI } else { LO };
        true
    }

    fn get_output(&mut self) -> u8 {
        self.state
    }
}

struct Conjunction {
    inputs: HashSet<String>,
    hi_senders: HashSet<String>,
    lo_senders: HashSet<String>,
    output: u8
}

impl Module for Conjunction {
    fn enable_input(&mut self, module: &String) {
        self.inputs.insert(module.clone());
        self.lo_senders.insert(module.clone());
    }

    fn receive(&mut self, module: &String, input: u8) -> bool {
        if input == HI {
            self.lo_senders.remove(module);
            self.hi_senders.insert(module.clone());
        } else {
            self.lo_senders.insert(module.clone());
            self.hi_senders.remove(module);
        }
        
        self.output = if self.hi_senders.len() == self.inputs.len() { LO } else { HI };
        true
    }

    fn get_output(&mut self) -> u8 {
        dbg!(&self.inputs);
        // dbg!(&self.hi_senders, &self.lo_senders);
        self.output
    }
}

struct Broadcast {
    output: u8,
}

impl Module for Broadcast {
    fn enable_input(&mut self, _: &String) {
        // noop
    }

    fn receive(&mut self, _: &String, input: u8) -> bool {
        self.output = input;
        true
    }

    fn get_output(&mut self) -> u8 {
        self.output
    }
}

fn parse(input: &str) -> (ModuleMap, ConnectionMap) {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    // Create modules
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let identifier = parts.next().unwrap();
        let outputs: Vec<String> = parts.next().unwrap().split(", ").map(|s| s.to_string()).collect();

        if identifier.starts_with("%") {
            let id = identifier[1..].to_string();
            modules.insert(
                id.clone(),
                Box::new(FlipFlop {
                    state: LO
                }));
            connections.insert(id, outputs);
        } else if identifier.starts_with("&") {
            let id = identifier[1..].to_string();
            modules.insert(
                id.clone(),
                Box::new(Conjunction {
                    inputs: HashSet::new(),
                    hi_senders: HashSet::new(),
                    lo_senders: HashSet::new(),
                    output: HI
                }));
            connections.insert(id, outputs);
        } else if identifier == "broadcaster" {
            modules.insert(
                identifier.to_string(),
                Box::new(Broadcast {
                    output: LO
                })); 
            connections.insert(identifier.to_string(), outputs);
        } else {
            panic!("Unknown module type: {}", identifier);
        }
    }

    for (from, to) in connections.clone() {
        for to_id in to {
            if let Some(to_module) = modules.get_mut(&to_id) {
                println!("Enabling {} to {}", from, to_id);
                to_module.enable_input(&from);
            }
        }
    }

    (modules, connections)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut modules, connections) = parse(input);

    // Queue of sending module, receiving module, value sent
    let mut queue: VecDeque<(String, String, u8)> = VecDeque::new();

    let mut hi_sent = 0;
    let mut lo_sent = 0;
    for _ in 0..1000 {
        let broadcaster = modules.get_mut("broadcaster").unwrap();
        broadcaster.receive(&"button".to_string(), LO);
        let output = broadcaster.get_output();
        for destination in connections.get("broadcaster").unwrap() {
            if output == HI {
                hi_sent += 1;
            } else {
                lo_sent += 1;
            }

            queue.push_back(("broadcaster".to_string(), destination.clone(), output));
        }

        while !queue.is_empty() {
            let (sender, receiver, value) = queue.pop_front().unwrap();
            if let Some(module) = modules.get_mut(&receiver) {
                if module.receive(&sender, value) {
                    let output = module.get_output();
                    for destination in connections.get(&receiver).unwrap() {
                        if value == HI {
                            hi_sent += 1;
                        } else {
                            lo_sent += 1;
                        }
    
                        queue.push_back((receiver.clone(), destination.clone(), output));
                    }
                }
            } else {
                // println!("Unknown module: {}", sender);
            }
        }
    }

    Some(hi_sent * lo_sent)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
