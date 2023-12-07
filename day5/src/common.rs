use std::str::{FromStr, Lines};

pub type Val = u64;

pub struct MappingRule {
    dest: Val,
    src: Val,
    count: Val,
}

impl MappingRule {
    fn map(&self, input: Val) -> Option<Val> {
        if self.src <= input && self.src + self.count > input {
            Some(input + self.dest - self.src)
        } else {
            None
        }
    }
}

pub struct Mapping {
    rules: Vec<MappingRule>,
}

impl Mapping {
    fn map(&self, input: Val) -> Val {
        self.rules
            .iter()
            .filter_map(|rule| rule.map(input))
            .next()
            .unwrap_or(input)
    }
}

pub fn get_seeds(lines: &mut Lines) -> Vec<Val> {
    let line = lines.next().unwrap();

    line[7..]
        .split(' ')
        .map(|seed_str| Val::from_str(seed_str).expect("A seed was not a number"))
        .collect::<Vec<Val>>()
}

pub fn get_mappings(lines: &mut Lines) -> Vec<Mapping> {
    let mut mappings = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        } else if line.contains("map:") {
            mappings.push(Mapping { rules: Vec::new() });
        } else {
            let rule: Vec<Val> = line
                .split(" ")
                .map(|s| Val::from_str(s).expect("Should be numberic"))
                .collect();

            assert_eq!(rule.len(), 3, "Rule should have 3 Values");

            mappings
                .last_mut()
                .expect("Should have a mapping")
                .rules
                .push(MappingRule {
                    dest: rule[0],
                    src: rule[1],
                    count: rule[2],
                })
        }
    }

    mappings
}

pub fn get_shortest_path(seeds: impl Iterator<Item=Val>, mappings: &Vec<Mapping>) -> Val {
    seeds
        .map(|seed| mappings.iter().fold(seed, |id, mapping| mapping.map(id)))
        .min()
        .expect("no path")
}
