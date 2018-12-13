use itertools::Itertools;
use std::collections::BTreeSet;

type Rule = ([bool; 5], bool);

fn parse_input(input: &str) -> (BTreeSet<isize>, Vec<Rule>) {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref HEADER_PATTERN: Regex = Regex::new(r"initial state: ([#\.]+)").unwrap();
        static ref RULE_PATTERN: Regex =
            Regex::new(r"([#\.])([#\.])([#\.])([#\.])([#\.]) => ([#\.])").unwrap();
    }

    let initial_state = HEADER_PATTERN
        .captures_iter(input)
        .filter_map(|caps| {
            Some(
                caps.get(1)?
                    .as_str()
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| if c == '#' { Some(i as isize) } else { None })
                    .collect::<BTreeSet<_>>(),
            )
        })
        .nth(0)
        .expect("It has a header");

    let rules = RULE_PATTERN
        .captures_iter(input)
        // .inspect(|caps| println!("{:?}", &caps))
        .filter_map(|caps| {
            Some((
                [
                    caps.get(1)?.as_str() == "#",
                    caps.get(2)?.as_str() == "#",
                    caps.get(3)?.as_str() == "#",
                    caps.get(4)?.as_str() == "#",
                    caps.get(5)?.as_str() == "#",
                ],
                caps.get(6)?.as_str() == "#",
            ))
        })
        .collect();

    (initial_state, rules)
}

fn front_and_back<'a, T: Clone>(data: &'a BTreeSet<T>) -> (Option<&'a T>, Option<&'a T>) {
    let mut it = data.iter();
    let first = it.next();
    let last = it.next_back().or(first);
    (first, last)
}

pub fn find_pattern_sum(input: &str, generations: usize) -> Option<i64> {
    let (initial_state, rules) = parse_input(input);

    (0..)
        .scan(initial_state, |state, i| {
            if let (Some(first), Some(last)) = front_and_back(&state) {
                *state = (first - 2..last + 2)
                    .filter(|i| {
                        rules.iter().cloned().any(|(rule, should_insert)| {
                            should_insert
                                && [
                                    state.contains(&(i - 2)),
                                    state.contains(&(i - 1)),
                                    state.contains(&i),
                                    state.contains(&(i + 1)),
                                    state.contains(&(i + 2)),
                                ] == rule
                        })
                    })
                    .collect();
                Some((i, state.iter().map(|x| *x as i64).sum()))
            } else {
                None
            }
        })
        .tuple_windows::<(_, _, _, _)>()
        .find_map(|((_, sum_a), (_, sum_b), (_, sum_c), (i, sum_d))| {
            // Stop either when we reach the generation
            if i + 1 == generations {
                Some(sum_d)
            // Or when we find a sum that increases by the same amount for three turns
            } else if sum_d - sum_c == sum_c - sum_b && sum_c - sum_b == sum_b - sum_a {
                println!("Repeating sum {} at {}", sum_b - sum_a, i);
                Some(sum_d + (sum_d - sum_c) * (generations - 1 - i) as i64)
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");
    const TEST_INPUT: &str = include_str!("test_input");

    #[test]
    fn part1_works() {
        assert_eq!(find_pattern_sum(TEST_INPUT, 20), Some(325));
        assert_eq!(find_pattern_sum(INPUT, 20), Some(3230));
    }

    #[test]
    fn part2_works() {
        assert_eq!(find_pattern_sum(INPUT, 50_000_000_000), Some(4_400_000_000_304));
    }
}
