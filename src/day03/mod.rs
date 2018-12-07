use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Debug)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn overlaps(&self, other: &Claim) -> bool {
        self.left < other.left + other.width
            && self.left + self.width > other.left
            && self.top < other.top + other.height
            && self.top + self.height > other.top
    }
}

fn claims_iter<'a>(input: &'a str) -> impl Iterator<Item = Claim> + 'a {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();
    }

    PATTERN.captures_iter(input).filter_map(|caps| {
        Some(Claim {
            id: caps.get(1)?.as_str().parse().ok()?,
            left: caps.get(2)?.as_str().parse().ok()?,
            top: caps.get(3)?.as_str().parse().ok()?,
            width: caps.get(4)?.as_str().parse().ok()?,
            height: caps.get(5)?.as_str().parse().ok()?,
        })
    })
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    for Claim { left, top, width, height, .. } in claims_iter(input) {
        for x in left..left + width {
            for y in top..top + height {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    map.values().filter(|n| **n >= 2).count()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    // Pre-collect the claims, since we need to access them very frequently
    let claims: Vec<Claim> = claims_iter(input).collect();

    'claim: for (i, claim_a) in claims.iter().enumerate() {
        for (j, claim_b) in claims.iter().enumerate() {
            if i != j {
                if claim_a.overlaps(claim_b) {
                    continue 'claim;
                }
            }
        }

        return claim_a.id;
    }

    panic!("No solution found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 4);
        assert_eq!(part1(INPUT), 121_163);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 3);
        assert_eq!(part2(INPUT), 943);
    }
}
