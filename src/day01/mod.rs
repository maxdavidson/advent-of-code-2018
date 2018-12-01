use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::iter::{once, Iterator};
use std::str::FromStr;

fn iter_nums<'a, T: FromStr>(input: &'a str) -> impl Iterator<Item = T> + 'a {
    lazy_static! {
        static ref NUMBERS: Regex = Regex::new(r"(-|\+)?\d+").unwrap();
    }

    NUMBERS
        .find_iter(input)
        .filter_map(|chars| chars.as_str().parse::<T>().ok())
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i32 {
    iter_nums::<i32>(input).sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i32 {
    // We need to collect the values so we can cycle through them
    let nums = iter_nums::<i32>(input).collect::<Vec<_>>();

    once(&0)
        .chain(nums.iter().cycle())
        // Compute the accumulated sum
        .scan(0, |acc, num| {
            *acc += num;
            Some(*acc)
        })
        // Determine whether the value has been seen before
        .scan(HashSet::<i32>::new(), |seen, num| {
            let was_not_present = seen.insert(num);
            if was_not_present {
                Some(None)
            } else {
                Some(Some(num))
            }
        })
        .flatten()
        .nth(0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("+1, +1, +1"), 3);
        assert_eq!(part1("+1, +1, -2"), 0);
        assert_eq!(part1("-1, -2, -3"), -6);
        assert_eq!(part1(INPUT), 439);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("+1, -1"), 0);
        assert_eq!(part2("+3, +3, +4, -2, -4"), 10);
        assert_eq!(part2("-6, +3, +8, +5, -6"), 5);
        assert_eq!(part2("+7, +7, -2, -7, -4"), 14);
        assert_eq!(part2(INPUT), 124_645);
    }
}
