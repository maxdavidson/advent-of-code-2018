#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let digits = input.chars().filter_map(|c| c.to_digit(10));
    let shifted_digits = digits.clone().chain(digits.clone()).skip(1);

    digits
        .zip(shifted_digits)
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum()
}

#[allow(dead_code)]
pub fn part2_v1(input: &str) -> u32 {
    let digits = input.chars().filter_map(|c| c.to_digit(10));
    let shifted_digits = digits
        .clone()
        .chain(digits.clone())
        .skip(digits.clone().count() / 2);

    digits
        .zip(shifted_digits)
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum()
}

#[allow(dead_code)]
pub fn part2_v2(input: &str) -> u32 {
    let digits = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    let len = digits.len();
    let mut sum: u32 = 0;

    for i in 0..len {
        let a = digits[i];
        let b = digits[(i + len / 2) % len];
        if a == b {
            sum += a;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("1122"), 3);
        assert_eq!(part1("1111"), 4);
        assert_eq!(part1("1234"), 0);
        assert_eq!(part1("91212129"), 9);
        assert_eq!(part1(INPUT), 1031);
    }

    #[test]
    fn part2_v1_works() {
        assert_eq!(part2_v1("1212"), 6);
        assert_eq!(part2_v1("1221"), 0);
        assert_eq!(part2_v1("123425"), 4);
        assert_eq!(part2_v1("123123"), 12);
        assert_eq!(part2_v1("12131415"), 4);
        assert_eq!(part2_v1(INPUT), 1080);
    }

    #[test]
    fn part2_v2_works() {
        assert_eq!(part2_v2("1212"), 6);
        assert_eq!(part2_v2("1221"), 0);
        assert_eq!(part2_v2("123425"), 4);
        assert_eq!(part2_v2("123123"), 12);
        assert_eq!(part2_v2("12131415"), 4);
        assert_eq!(part2_v2(INPUT), 1080);
    }
}
