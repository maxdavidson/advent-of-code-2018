use itertools::Itertools;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1_v1(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|line| {
            line.chars()
                .fold(HashMap::<_, u64>::new(), |mut freq, x| {
                    *freq.entry(x).or_insert(0) += 1;
                    freq
                })
                .into_iter()
                .map(|(_, count)| count)
                .unique()
        })
        .fold(HashMap::<_, u64>::new(), |mut freq, x| {
            *freq.entry(x).or_insert(0) += 1;
            freq
        })
        .into_iter()
        .filter_map(|(k, v)| if k == 2 || k == 3 { Some(v) } else { None })
        .product()
}

#[allow(dead_code)]
pub fn part1_v2(input: &str) -> u64 {
    let mut twos = 0u64;
    let mut threes = 0u64;

    for line in input.lines() {
        let mut freq = HashMap::new();

        for c in line.chars() {
            *freq.entry(c).or_insert(0u64) += 1;
        }

        for unique_freq in freq.values().unique() {
            match *unique_freq {
                2 => twos += 1,
                3 => threes += 1,
                _ => {}
            }
        }
    }

    twos * threes
}

#[allow(dead_code)]
pub fn part2_v1(input: &str) -> String {
    input
        .lines()
        .tuple_combinations::<(_, _)>()
        .find_map(|(line_a, line_b)| {
            let common_chars = line_a
                .chars()
                .zip_eq(line_b.chars())
                .filter_map(|(c_a, c_b)| if c_a == c_b { Some(c_a) } else { None })
                .collect::<String>();

            if common_chars.len() == line_a.len() - 1 {
                Some(common_chars)
            } else {
                None
            }
        })
        .expect("No solution found!")
}

#[allow(dead_code)]
pub fn part2_v2(input: &str) -> String {
    for (line_a, line_b) in input.lines().tuple_combinations::<(_, _)>() {
        if line_a.len() != line_b.len() {
            continue;
        }

        let mut common_chars = String::new();

        for (c_a, c_b) in line_a.chars().zip(line_b.chars()) {
            if c_a == c_b {
                common_chars.push(c_a);
            }
        }

        if common_chars.len() == line_a.len() - 1 {
            return common_chars;
        }
    }

    panic!("No solution found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1_v1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
        assert_eq!(part1_v1(INPUT), 6175);
    }

    #[test]
    fn part1_v2_works() {
        assert_eq!(part1_v2("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
        assert_eq!(part1_v2(INPUT), 6175);
    }

    #[test]
    fn part2_v1_works() {
        assert_eq!(part2_v1("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), "fgij");
        assert_eq!(part2_v1(INPUT), "asgwjcmzredihqoutcylvzinx");
    }

    #[test]
    fn part2_v2_works() {
        assert_eq!(part2_v2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), "fgij");
        assert_eq!(part2_v2(INPUT), "asgwjcmzredihqoutcylvzinx");
    }
}
