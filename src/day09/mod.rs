use std::collections::VecDeque;

trait Rotate<N> {
    fn rotate(&mut self, n: N);
}

impl<T> Rotate<isize> for VecDeque<T> {
    fn rotate(&mut self, n: isize) {
        if !self.is_empty() {
            use std::cmp::Ordering;
            match n.cmp(&0) {
                Ordering::Greater => {
                    for _ in 0..n {
                        let front = self.pop_front().unwrap();
                        self.push_back(front);
                    }
                }
                Ordering::Less => {
                    for _ in 0..-n {
                        let back = self.pop_back().unwrap();
                        self.push_front(back);
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn max_score(player_count: usize, last_marble_value: usize) -> usize {
    let mut player_scores = vec![0; player_count];
    let mut marbles = VecDeque::with_capacity(last_marble_value);

    marbles.push_front(0);

    for marble_number in 1..=last_marble_value {
        if marble_number % 23 == 0 {
            marbles.rotate(7);
            let player_index = marble_number % player_count;
            let removed_marble_number = marbles.pop_front().unwrap();
            player_scores[player_index] += marble_number + removed_marble_number;
            marbles.rotate(-1)
        } else {
            marbles.rotate(-1);
            marbles.push_front(marble_number);
        }
    }

    player_scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAYER_COUNT: usize = 491;
    const MAX_MARBLE_VALUE: usize = 71058;

    #[test]
    fn test_cases() {
        assert_eq!(max_score(9, 25), 32);
        assert_eq!(max_score(10, 1618), 8317);
        assert_eq!(max_score(13, 7999), 146_373);
        assert_eq!(max_score(17, 1104), 2764);
        assert_eq!(max_score(21, 6111), 54718);
        assert_eq!(max_score(30, 5807), 37305);
    }

    #[test]
    fn part1() {
        assert_eq!(max_score(PLAYER_COUNT, MAX_MARBLE_VALUE), 361_466);
    }

    #[test]
    fn part2() {
        assert_eq!(max_score(PLAYER_COUNT, 100 * MAX_MARBLE_VALUE), 2_945_918_550);
    }
}
