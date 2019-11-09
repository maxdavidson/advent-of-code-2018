use std::collections::{HashMap, VecDeque};

#[derive(Hash, Eq, PartialEq, Clone)]
struct Position {
    x: isize,
    y: isize,
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Position {
    fn moved_by(&self, direction: &Direction) -> Position {
        let Position { x, y } = *self;
        match direction {
            Direction::North => Position { x, y: y + 1 },
            Direction::South => Position { x, y: y - 1 },
            Direction::West => Position { x: x - 1, y },
            Direction::East => Position { x: x + 1, y },
        }
    }
}

enum Token {
    Direction(Direction),
    Separator,
    OpenParenthesis,
    CloseParenthesis,
}

fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    input.chars().filter_map(|c| match c {
        'N' => Some(Token::Direction(Direction::North)),
        'S' => Some(Token::Direction(Direction::South)),
        'E' => Some(Token::Direction(Direction::East)),
        'W' => Some(Token::Direction(Direction::West)),
        '|' => Some(Token::Separator),
        '(' => Some(Token::OpenParenthesis),
        ')' => Some(Token::CloseParenthesis),
        _ => None,
    })
}

fn compute_distances(input: &str) -> impl Iterator<Item = usize> {
    let tokens = tokenize(input);

    let mut positions = VecDeque::new();
    let mut distances = HashMap::new();
    let mut position = Position { x: 0, y: 0 };

    for token in tokens {
        match token {
            Token::OpenParenthesis => {
                positions.push_back(position.clone());
            }

            Token::CloseParenthesis => {
                position = positions.pop_back().unwrap();
            }

            Token::Separator => {
                position = positions.back().unwrap().clone();
            }

            Token::Direction(direction) => {
                let next_position = position.moved_by(&direction);
                let next_distance = *distances.get(&position).unwrap_or(&0usize) + 1;

                // Replace the current distance
                match distances.get(&next_position) {
                    // Ignore if the best distance is shorter than the nest
                    Some(best_distance) if *best_distance <= next_distance => {}
                    _ => {
                        distances.insert(next_position.clone(), next_distance);
                    }
                };

                position = next_position;
            }
        }
    }

    distances.into_iter().map(|(_, v)| v)
}

pub fn part1(input: &str) -> usize {
    compute_distances(input).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    compute_distances(input).filter(|distance| *distance >= 1000).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("^WNE$"), 3);
        assert_eq!(part1("^ENWWW(NEEE|SSE(EE|N))$"), 10);
        assert_eq!(part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18);
        assert_eq!(part1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"), 23);
        assert_eq!(part1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"), 31);
        assert_eq!(part1(INPUT), 3810);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 8615);
    }
}
