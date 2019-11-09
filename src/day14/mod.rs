use std::char::from_digit;

fn get_digits(input: &str) -> Vec<u8> {
    input.chars().filter_map(|c| c.to_digit(10)).map(|x| x as u8).collect()
}

pub fn part1(skip: usize) -> String {
    let mut scores: Vec<u8> = vec![3, 7];
    let mut a: usize = 0;
    let mut b: usize = 1;

    while scores.len() < skip + 10 {
        let sum = scores[a] + scores[b];

        if sum < 10 {
            scores.push(sum);
        } else {
            scores.push(sum / 10);
            scores.push(sum % 10);
        }

        a = (a + 1 + (scores[a] as usize)) % scores.len();
        b = (b + 1 + (scores[b] as usize)) % scores.len();
    }

    scores[skip..skip + 10].iter().filter_map(|x| from_digit(u32::from(*x), 10)).collect()
}

pub fn part2(pattern: &str) -> usize {
    let score_sequence = get_digits(pattern);

    let mut scores: Vec<u8> = vec![3, 7];

    let mut a: usize = 0;
    let mut b: usize = 1;

    loop {
        if let Some((index, _)) = scores
            .windows(score_sequence.len())
            .rev()
            .take(2)
            .enumerate()
            .find(|(_, window)| window.starts_with(&score_sequence))
        {
            break scores.len() - score_sequence.len() - index;
        }

        let sum = scores[a] + scores[b];

        if sum < 10 {
            scores.push(sum);
        } else {
            scores.push(sum / 10);
            scores.push(sum % 10);
        }

        a = (a + 1 + (scores[a] as usize)) % scores.len();
        b = (b + 1 + (scores[b] as usize)) % scores.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(5), "0124515891");
        assert_eq!(part1(9), "5158916779");
        assert_eq!(part1(18), "9251071085");
        assert_eq!(part1(2018), "5941429882");
        assert_eq!(part1(607_331), "8610321414");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("51589"), 9);
        assert_eq!(part2("01245"), 5);
        assert_eq!(part2("92510"), 18);
        assert_eq!(part2("59414"), 2018);
        assert_eq!(part2("607331"), 20_258_123);
    }
}
