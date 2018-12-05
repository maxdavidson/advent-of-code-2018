fn reacts(a: &char, b: &char) -> bool {
    a.is_ascii_lowercase() != b.is_ascii_lowercase() && a.eq_ignore_ascii_case(&b)
}

fn collapse(input: &str) -> impl Iterator<Item = char> {
    let mut chars: Vec<_> = input.chars().filter(char::is_ascii_alphabetic).collect();
    let mut i: usize = 0;

    while i + 1 < chars.len() {
        if reacts(&chars[i], &chars[i + 1]) {
            chars.remove(i);
            chars.remove(i);
            if i != 0 {
                i -= 1
            }
        } else {
            i += 1;
        }
    }

    chars.into_iter()
}

pub fn part1(input: &str) -> usize {
    collapse(input).count()
}

pub fn part2(input: &str) -> usize {
    use std::char;

    let all_units = (0x61..=0x7a).filter_map(char::from_u32);

    all_units
        .map(|unit| {
            let filtered_input = input.replace(|c: char| c.eq_ignore_ascii_case(&unit), "");

            collapse(&filtered_input).count()
        })
        .min()
        .expect("No solution found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("aA"), 0);
        assert_eq!(part1("abBA"), 0);
        assert_eq!(part1("abAB"), 4);
        assert_eq!(part1("aabAAb"), 6);
        assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
        assert_eq!(part1(INPUT), 9562);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 4934);
    }

}
