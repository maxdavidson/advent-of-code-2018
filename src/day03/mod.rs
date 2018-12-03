use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn claims_iter<'a>(input: &'a str) -> impl Iterator<Item = Claim> + 'a {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();
    }

    PATTERN.captures_iter(input).filter_map(|caps| {
        if let (Some(id), Some(left), Some(top), Some(width), Some(height)) = (
            caps.get(1).and_then(|c| u32::from_str(c.as_str()).ok()),
            caps.get(2).and_then(|c| u32::from_str(c.as_str()).ok()),
            caps.get(3).and_then(|c| u32::from_str(c.as_str()).ok()),
            caps.get(4).and_then(|c| u32::from_str(c.as_str()).ok()),
            caps.get(5).and_then(|c| u32::from_str(c.as_str()).ok()),
        ) {
            #[cfg_attr(rustfmt, rustfmt::skip)]
                Some(Claim { id, left, top, width, height })
        } else {
            None
        }
    })
}

fn claim_coords_iter(claim: &Claim) -> impl Iterator<Item = (u32, u32)> {
    #[cfg_attr(rustfmt, rustfmt::skip)]
    let Claim { left, top, width, height, .. } = *claim;
    (left..left + width).flat_map(move |x| (top..top + height).map(move |y| (x, y)))
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    #[cfg_attr(rustfmt, rustfmt::skip)]
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
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    for claim in claims_iter(input) {
        for coord in claim_coords_iter(&claim) {
            *map.entry(coord).or_insert(0) += 1;
        }
    }

    'claim: for claim in claims_iter(input) {
        for coord in claim_coords_iter(&claim) {
            if let Some(overlaps) = map.get(&coord) {
                if *overlaps >= 2 {
                    continue 'claim;
                }
            }
        }

        return claim.id;
    }

    panic!("No solution found!");
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
