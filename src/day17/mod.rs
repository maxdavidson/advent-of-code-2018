use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    y: isize,
    x: isize,
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
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Clay,
    FlowingWater,
    SettledWater,
}

fn parse(input: &str) -> HashMap<Position, Tile> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"([xy])=(\d+), [xy]=(\d+)..(\d+)").unwrap();
    }

    PATTERN
        .captures_iter(input)
        .filter_map(|caps| {
            let symbol = caps.get(1)?.as_str();
            let index = caps.get(2)?.as_str().parse().ok()?;
            let range = caps.get(3)?.as_str().parse().ok()?..=caps.get(4)?.as_str().parse().ok()?;

            Some(match symbol {
                "x" => ((index..=index), range),
                "y" => (range, (index..=index)),
                _ => panic!("unexpected symbol!"),
            })
        })
        .flat_map(|(x_range, y_range)| {
            x_range.cartesian_product(y_range).map(|(x, y)| (Position { x, y }, Tile::Clay))
        })
        .collect()
}

#[allow(dead_code)]
fn compute_bounds(positions: impl Iterator<Item = Position>) -> Option<(Position, Position)> {
    use core::cmp::{max, min};
    positions.fold(None, |current_bounds, current_position| {
        if let Some((min_position, max_position)) = current_bounds {
            Some((
                Position {
                    x: min(min_position.x, current_position.x),
                    y: min(min_position.y, current_position.y),
                },
                Position {
                    x: max(max_position.x, current_position.x),
                    y: max(max_position.y, current_position.y),
                },
            ))
        } else {
            Some((current_position, current_position))
        }
    })
}

#[allow(dead_code)]
fn draw(map: &HashMap<Position, Tile>) {
    use core::cmp::{max, min};

    let bounds = map.keys().fold(None, |current_bounds, position| {
        let Position { x, y } = *position;
        if let Some((min_x, min_y, max_x, max_y)) = current_bounds {
            Some((min(min_x, x), min(min_y, y), max(max_x, x), max(max_y, y)))
        } else {
            Some((x, y, x, y))
        }
    });

    if let Some((min_x, min_y, max_x, max_y)) = bounds {
        for y in min_y - 1..=max_y + 1 {
            let line = (min_x - 1..=max_x + 1)
                .map(move |x| {
                    let position = Position { x, y };
                    match map.get(&position) {
                        Some(Tile::FlowingWater) => '|',
                        Some(Tile::SettledWater) => '~',
                        Some(Tile::Clay) => '#',
                        None => '.',
                    }
                })
                .collect::<String>();

            println!("{}", line);
        }
    }
}

fn solve(map: &mut HashMap<Position, Tile>) {
    let (min_y, max_y) = map.keys().map(|position| position.y).minmax().into_option().unwrap();

    let mut stack = VecDeque::new();
    let starting_position = Position { x: 500, y: 0 };
    stack.push_back(starting_position);

    while let Some(position) = stack.pop_back() {
        map.insert(position, Tile::FlowingWater);
        let down_position = position.moved_by(Direction::Down);

        match map.get(&down_position) {
            None => {
                if position.y < max_y {
                    stack.push_back(position);
                    stack.push_back(down_position);
                }
            }

            Some(Tile::FlowingWater) => {}

            Some(Tile::Clay) | Some(Tile::SettledWater) => {
                let search = |direction| {
                    let mut position = position;
                    loop {
                        let next_position = position.moved_by(direction);
                        let next_tile = map.get(&next_position);
                        let down_tile = map.get(&position.moved_by(Direction::Down));

                        if next_tile == Some(&Tile::Clay) {
                            break (position.x, true);
                        } else if down_tile == None || down_tile == Some(&Tile::FlowingWater) {
                            break (position.x, false);
                        }

                        position = next_position;
                    }
                };

                let y = position.y;
                let (left_x, hit_left_wall) = search(Direction::Left);
                let (right_x, hit_right_wall) = search(Direction::Right);

                if hit_left_wall && hit_right_wall {
                    for x in left_x..=right_x {
                        map.insert(Position { x, y }, Tile::SettledWater);
                    }
                } else {
                    for x in left_x..=right_x {
                        map.insert(Position { x, y }, Tile::FlowingWater);
                    }

                    if !hit_left_wall {
                        stack.push_back(Position { x: left_x, y });
                    }

                    if !hit_right_wall {
                        stack.push_back(Position { x: right_x, y });
                    }
                }
            }
        }
    }

    map.retain(|position, _| min_y <= position.y && position.y <= max_y);
}

pub fn part1(input: &str) -> usize {
    let mut map = parse(input);
    solve(&mut map);
    map.values().filter(|tile| **tile == Tile::FlowingWater || **tile == Tile::SettledWater).count()
}

pub fn part2(input: &str) -> usize {
    let mut map = parse(input);
    solve(&mut map);
    map.values().filter(|tile| **tile == Tile::SettledWater).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input");
    const INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 57);
        assert_eq!(part1(INPUT), 39649);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 29);
        assert_eq!(part2(INPUT), 28872);
    }
}
