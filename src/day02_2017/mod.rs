use num_bigint::BigInt;
use num_traits::Zero;

#[allow(dead_code)]
pub fn part1(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            line.trim()
                .split_whitespace()
                .filter_map(|s| BigInt::parse_bytes(s.as_bytes(), 10))
                .fold(None::<(_, _)>, |acc, val| {
                    if let Some((min, max)) = acc {
                        let new_min = if val < min { val.clone() } else { min };
                        let new_max = if max < val { val } else { max };
                        Some((new_min, new_max))
                    } else {
                        Some((val.clone(), val))
                    }
                })
                .map(|(min, max)| max - min)
        })
        .sum::<BigInt>()
        .to_string()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .filter_map(|s| BigInt::parse_bytes(s.as_bytes(), 10))
                .collect::<Vec<_>>()
        })
        .filter_map(|numbers| {
            numbers.iter().enumerate().find_map(|(i, a)| {
                numbers.iter().enumerate().find_map(|(j, b)| {
                    if i != j && a % b == Zero::zero() {
                        Some(a / b)
                    } else {
                        None
                    }
                })
            })
        })
        .sum::<BigInt>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("5 1 9 5\n7 5 3\n2 4 6 8\n"), "18");
        assert_eq!(part1(INPUT), "54426");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("5 9 2 8\n9 4 7 3\n3 8 6 5\n"), "9");
        assert_eq!(part2(INPUT), "333");
    }

}
