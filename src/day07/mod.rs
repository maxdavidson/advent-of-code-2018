use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

fn pairs_iter<'a>(input: &'a str) -> impl Iterator<Item = (char, char)> + 'a {
    input.trim().lines().filter_map(|line| {
        Some((
            // parent
            line.chars().nth(5)?,
            // id
            line.chars().nth(36)?,
        ))
    })
}

struct Schedule {
    parents: HashMap<char, HashSet<char>>,
    children: HashMap<char, HashSet<char>>,
}

impl Schedule {
    fn new(pairs: impl Iterator<Item = (char, char)>) -> Schedule {
        let mut parents = HashMap::new();
        let mut children = HashMap::new();

        for (parent, child) in pairs {
            children.entry(parent).or_insert_with(HashSet::new).insert(child);
            parents.entry(parent).or_insert_with(HashSet::new);
            parents.entry(child).or_insert_with(HashSet::new).insert(parent);
        }

        Schedule { parents, children }
    }
}

impl Iterator for Schedule {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let node = *self
            .parents
            .iter()
            .filter(|(_, parents)| parents.is_empty())
            .map(|(child, _)| child)
            .min()?;

        if let Some(node_children) = self.children.get(&node) {
            for child in node_children {
                self.parents.entry(*child).or_insert_with(HashSet::new).remove(&node);
            }
        }

        self.parents.remove(&node);

        Some(node)
    }
}

pub fn part1(input: &str) -> String {
    let pairs = pairs_iter(input);
    Schedule::new(pairs).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("input");
    const TEST_INPUT: &'static str = include_str!("test_input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), "CABDFE");
        assert_eq!(part1(INPUT), "FDSEGJLPKNRYOAMQIUHTCVWZXB");
    }
}
