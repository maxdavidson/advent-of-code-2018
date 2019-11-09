use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn moved_by(self, direction: Direction) -> Position {
        let Position { x, y } = self;
        match direction {
            Direction::Up => Position { x, y: y - 1 },
            Direction::Down => Position { x, y: y + 1 },
            Direction::Left => Position { x: x - 1, y },
            Direction::Right => Position { x: x + 1, y },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Track {
    Vertical,
    Horizontal,
    CurveRight,
    CurveLeft,
    Intersection,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum NextIntersection {
    GoStraight,
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Eq, PartialEq)]
struct Cart {
    direction: Direction,
    next_intersection: NextIntersection,
}

impl Cart {
    fn new(direction: Direction) -> Cart {
        Cart { direction, next_intersection: NextIntersection::TurnLeft }
    }

    fn turned_by(&self, track: Track) -> Cart {
        let direction = match track {
            Track::Intersection => match self.next_intersection {
                NextIntersection::GoStraight => self.direction,
                NextIntersection::TurnLeft => match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                },
                NextIntersection::TurnRight => match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                },
            },

            Track::CurveRight => match self.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },

            Track::CurveLeft => match self.direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },

            Track::Vertical | Track::Horizontal => self.direction,
        };

        let next_intersection = {
            match (track, self.next_intersection) {
                (Track::Intersection, NextIntersection::GoStraight) => NextIntersection::TurnRight,
                (Track::Intersection, NextIntersection::TurnLeft) => NextIntersection::GoStraight,
                (Track::Intersection, NextIntersection::TurnRight) => NextIntersection::TurnLeft,
                _ => self.next_intersection,
            }
        };

        Cart { direction, next_intersection }
    }
}

trait Pop<T> {
    fn pop(&mut self) -> Option<T>;
}

impl<K: Ord + Clone, V> Pop<(K, V)> for BTreeMap<K, V> {
    fn pop(&mut self) -> Option<(K, V)> {
        let key = self.keys().next()?.clone();
        let value = self.remove(&key)?;
        Some((key, value))
    }
}

fn parse(input: &str) -> (HashMap<Position, Track>, BTreeMap<Position, Cart>) {
    let mut tracks = HashMap::new();
    let mut carts = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let position = Position { x, y };
            match c {
                '|' => {
                    tracks.insert(position, Track::Vertical);
                }
                '-' => {
                    tracks.insert(position, Track::Horizontal);
                }
                '/' => {
                    tracks.insert(position, Track::CurveRight);
                }
                '\\' => {
                    tracks.insert(position, Track::CurveLeft);
                }
                '+' => {
                    tracks.insert(position, Track::Intersection);
                }
                'v' => {
                    carts.insert(position, Cart::new(Direction::Down));
                    tracks.insert(position, Track::Vertical);
                }
                '^' => {
                    carts.insert(position, Cart::new(Direction::Up));
                    tracks.insert(position, Track::Vertical);
                }
                '<' => {
                    carts.insert(position, Cart::new(Direction::Left));
                    tracks.insert(position, Track::Horizontal);
                }
                '>' => {
                    carts.insert(position, Cart::new(Direction::Right));
                    tracks.insert(position, Track::Horizontal);
                }
                _ => {}
            }
        }
    }

    (tracks, carts)
}

#[allow(dead_code)]
fn draw(tracks: &HashMap<Position, Track>, carts: &BTreeMap<Position, Cart>) {
    use core::cmp::{max, min};

    let bounds = tracks.keys().fold(None, |current_bounds, position| {
        let Position { x, y } = *position;
        if let Some((min_x, min_y, max_x, max_y)) = current_bounds {
            Some((min(min_x, x), min(min_y, y), max(max_x, x), max(max_y, y)))
        } else {
            Some((x, y, x, y))
        }
    });

    if let Some((min_x, min_y, max_x, max_y)) = bounds {
        for y in min_y..=max_y {
            let line = (min_x..=max_x)
                .map(move |x| {
                    let position = Position { x, y };

                    if let Some(cart) = carts.get(&position) {
                        match cart.direction {
                            Direction::Up => '^',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                            Direction::Right => '>',
                        }
                    } else if let Some(track) = tracks.get(&position) {
                        match track {
                            Track::CurveLeft => '\\',
                            Track::CurveRight => '/',
                            Track::Intersection => '+',
                            Track::Vertical => '|',
                            Track::Horizontal => '-',
                        }
                    } else {
                        ' '
                    }
                })
                .collect::<String>();

            println!("{}", line);
        }
    }
}

pub fn part1(input: &str) -> Position {
    let (tracks, mut carts) = parse(input);

    loop {
        let mut next_carts = BTreeMap::new();

        while let Some((position, cart)) = carts.pop() {
            let track = tracks.get(&position).cloned().expect("Derailed cart!");
            let cart = cart.turned_by(track);
            let position = position.moved_by(cart.direction);

            if carts.remove(&position).or_else(|| next_carts.remove(&position)).is_some() {
                return position;
            } else {
                next_carts.insert(position, cart);
            }
        }

        carts = next_carts;
    }
}

pub fn part2(input: &str) -> Position {
    let (tracks, mut carts) = parse(input);

    if carts.len() % 2 == 0 {
        panic!("Even number of carts");
    }

    while 1 < carts.len() {
        let mut next_carts = BTreeMap::new();

        while let Some((position, cart)) = carts.pop() {
            let track = tracks.get(&position).cloned().expect("Derailed cart!");
            let cart = cart.turned_by(track);
            let position = position.moved_by(cart.direction);

            if carts.remove(&position).or_else(|| next_carts.remove(&position)).is_none() {
                next_carts.insert(position, cart);
            }
        }

        carts = next_carts;
    }

    let (last_position, _) = carts.pop().unwrap();

    last_position
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");
    const TEST_INPUT: &str = include_str!("test_input");
    const TEST_INPUT_2: &str = include_str!("test_input_2");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), Position { x: 7, y: 3 });
        assert_eq!(part1(INPUT), Position { x: 83, y: 121 });
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT_2), Position { x: 6, y: 4 });
        assert_eq!(part2(INPUT), Position { x: 102, y: 144 });
    }
}
