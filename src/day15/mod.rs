use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::iter::once;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct Position {
    y: isize,
    x: isize,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    fn is_neighbor(&self, other: Position) -> bool {
        self.neighbors().any(|position| position == other)
    }

    fn neighbors(self) -> impl Iterator<Item = Position> {
        let Position { x, y } = self;
        once(Position { x, y: y - 1 })
            .chain(once(Position { x, y: y + 1 }))
            .chain(once(Position { x: x - 1, y }))
            .chain(once(Position { x: x + 1, y }))
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum UnitKind {
    Elf,
    Goblin,
}

struct Unit {
    kind: UnitKind,
    position: Position,
    is_alive: bool,
    health_points: isize,
    attack_power: isize,
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({}) @ {:?}", self.kind, self.health_points, self.position)
    }
}

#[derive(Debug)]
enum GameError {
    ElfDied,
    NoSolution,
}

type GameResult = Result<isize, GameError>;

#[allow(dead_code)]
fn draw(walls: &HashSet<Position>, units: &[Unit]) {
    use core::cmp::{max, min};

    let bounds = walls.iter().fold(None, |current_bounds, position| {
        let Position { x, y } = *position;
        if let Some((min_x, min_y, max_x, max_y)) = current_bounds {
            Some((min(min_x, x), min(min_y, y), max(max_x, x), max(max_y, y)))
        } else {
            Some((x, y, x, y))
        }
    });

    let units = units
        .iter()
        .filter(|unit| unit.is_alive)
        .map(|unit| (unit.position, unit))
        .collect::<HashMap<_, _>>();

    if let Some((min_x, min_y, max_x, max_y)) = bounds {
        for y in min_y..=max_y {
            let line = (min_x..=max_x)
                .map(|x| {
                    let position = Position { x, y };

                    if let Some(unit) = units.get(&position) {
                        match unit.kind {
                            UnitKind::Goblin => 'G',
                            UnitKind::Elf => 'E',
                        }
                    } else if walls.contains(&position) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>();

            println!("{}", line);
        }
    }
}

struct Grid {
    walls: HashSet<Position>,
    units: Vec<Unit>,
}

impl Grid {
    fn from_input(input: &str, elf_attack_power: isize) -> Grid {
        let mut walls = HashSet::new();
        let mut units = Vec::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Position { x: x as isize, y: y as isize };
                match c {
                    '#' => {
                        walls.insert(position);
                    }
                    'G' => {
                        units.push(Unit {
                            kind: UnitKind::Goblin,
                            position,
                            health_points: 200,
                            attack_power: 3,
                            is_alive: true,
                        });
                    }
                    'E' => {
                        units.push(Unit {
                            kind: UnitKind::Elf,
                            position,
                            health_points: 200,
                            attack_power: elf_attack_power,
                            is_alive: true,
                        });
                    }
                    _ => {}
                }
            }
        }

        Grid { walls, units }
    }

    fn play_game(&mut self, check_elf_death: bool) -> GameResult {
        for round in 0.. {
            // println!("");
            // println!("Round {}", round);
            // draw(&self.walls, &self.units);

            self.units.sort_by_key(|unit| unit.position);

            // To simplify the borrowing, use an index instead of a reference
            for i in 0..self.units.len() {
                if !self.units[i].is_alive {
                    continue;
                }

                // println!("");
                // println!("{:?}:", self.units[i]);

                if self
                    .units
                    .iter()
                    .filter(|target| self.units[i].kind != target.kind && target.is_alive)
                    .nth(0)
                    .is_none()
                {
                    let sum = self
                        .units
                        .iter()
                        .filter(|unit| unit.is_alive)
                        .map(|unit| unit.health_points)
                        .sum::<isize>();
                    return Ok(round * sum);
                }

                let occupied = self
                    .units
                    .iter()
                    .enumerate()
                    .filter(|(j, unit)| i != *j && unit.is_alive)
                    .map(|(_, unit)| unit.position)
                    .collect::<HashSet<_>>();

                let in_range = self
                    .units
                    .iter()
                    .filter(|target| self.units[i].kind != target.kind && target.is_alive)
                    .flat_map(|target| target.position.neighbors())
                    .filter(|position| {
                        !self.walls.contains(&position) && !occupied.contains(&position)
                    })
                    .collect::<HashSet<_>>();

                // println!("TARGETS IN RANGE: {:?}", &in_range);

                if !in_range.contains(&self.units[i].position) {
                    let next_move = {
                        let mut visiting = VecDeque::new();
                        let mut seen = HashSet::new();
                        let mut meta = HashMap::new();
                        visiting.push_back((self.units[i].position, 0));

                        while let Some((position, distance)) = visiting.pop_front() {
                            for neighbor_position in position.neighbors() {
                                if self.walls.contains(&position) || occupied.contains(&position) {
                                    continue;
                                }

                                let meta_entry = (distance + 1, position);
                                match meta.get(&neighbor_position) {
                                    Some(best_entry) if *best_entry <= meta_entry => {}
                                    _ => {
                                        meta.insert(neighbor_position, meta_entry);
                                    }
                                }

                                if seen.contains(&neighbor_position) {
                                    continue;
                                }

                                if !visiting.iter().any(|visit| visit.0 == neighbor_position) {
                                    visiting.push_back((neighbor_position, distance + 1));
                                }
                            }

                            seen.insert(position);
                        }

                        if let Some((_, mut closest_position)) = meta
                            .iter()
                            .filter(|(position, _)| in_range.contains(position))
                            .map(|(position, (distance, _))| (distance, position))
                            .min()
                        {
                            while let Some((_, parent_position)) =
                                meta.get(closest_position).filter(|(distance, _)| *distance > 1)
                            {
                                closest_position = parent_position;
                            }

                            Some(*closest_position)
                        } else {
                            None
                        }
                    };

                    if let Some(next_position) = next_move {
                        // println!("MOVE TO {:?}", next_position);
                        self.units[i].position = next_position;
                    }
                }

                if let Some((j, _)) = self
                    .units
                    .iter()
                    .enumerate()
                    .filter(|(_, target)| {
                        target.is_alive
                            && self.units[i].kind != target.kind
                            && self.units[i].position.is_neighbor(target.position)
                    })
                    .min_by_key(|(_, opponent)| (opponent.health_points, opponent.position))
                {
                    // println!("ATTACK {:?}", self.units[j]);

                    let attack_power = self.units[i].attack_power;
                    let opponent = &mut self.units[j];
                    opponent.health_points -= attack_power;
                    if opponent.health_points < 0 {
                        // println!("DEATH: {:?}", opponent);
                        opponent.is_alive = false;
                        if check_elf_death && opponent.kind == UnitKind::Elf {
                            return Err(GameError::ElfDied);
                        }
                    }
                }
            }
        }

        Err(GameError::NoSolution)
    }
}

pub fn part1(input: &str) -> isize {
    Grid::from_input(input, 3).play_game(false).unwrap()
}

pub fn part2(input: &str) -> isize {
    (4..)
        .find_map(|elf_attack_power| Grid::from_input(input, elf_attack_power).play_game(true).ok())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");
    const TEST_INPUT_1: &str = include_str!("test_input_1");
    const TEST_INPUT_2: &str = include_str!("test_input_2");
    const TEST_INPUT_3: &str = include_str!("test_input_3");
    const TEST_INPUT_4: &str = include_str!("test_input_4");
    const TEST_INPUT_5: &str = include_str!("test_input_5");
    const TEST_INPUT_6: &str = include_str!("test_input_6");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT_1), 27730);
        assert_eq!(part1(TEST_INPUT_2), 36334);
        assert_eq!(part1(TEST_INPUT_3), 39514);
        assert_eq!(part1(TEST_INPUT_4), 27755);
        assert_eq!(part1(TEST_INPUT_5), 28944);
        assert_eq!(part1(TEST_INPUT_6), 18740);
        assert_eq!(part1(INPUT), 206_236);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT_1), 4988);
        //assert_eq!(part2(TEST_INPUT_3), 31284);
        assert_eq!(part2(TEST_INPUT_4), 3478);
        assert_eq!(part2(TEST_INPUT_5), 6474);
        assert_eq!(part2(TEST_INPUT_6), 1140);
        assert_eq!(part2(INPUT), 88537);
    }
}
