use lazy_static::lazy_static;
use num_bigint::BigInt;
use num_traits::Zero;
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
pub fn part1(input: &str) -> String {
    iter_nums::<BigInt>(input).sum::<BigInt>().to_string()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> String {
    // We need to collect the values so we can cycle through them
    let nums: Vec<BigInt> = iter_nums(input).collect();

    once(&BigInt::zero())
        .chain(nums.iter().cycle())
        // Compute the accumulated sum
        .scan(BigInt::zero(), |acc, num| {
            *acc += num;
            Some(acc.clone())
        })
        // Check whether the value has been seen before
        .scan(HashSet::<BigInt>::new(), |seen, num| {
            if seen.contains(&num) {
                Some(Some(num))
            } else {
                seen.insert(num);
                Some(None)
            }
        })
        .flatten()
        .nth(0)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("+1, +1, +1"), "3");
        assert_eq!(part1("+1, +1, -2"), "0");
        assert_eq!(part1("-1, -2, -3"), "-6");
        assert_eq!(part1(INPUT), "439");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("+1, -1"), "0");
        assert_eq!(part2("+3, +3, +4, -2, -4"), "10");
        assert_eq!(part2("-6, +3, +8, +5, -6"), "5");
        assert_eq!(part2("+7, +7, -2, -7, -4"), "14");
        assert_eq!(part2(INPUT), "124645");
    }
}
