use itertools::Itertools;
use num::abs;
use std::collections::HashMap;
use std::iter::Iterator;

fn coords_iter<'a>(input: &'a str) -> impl Iterator<Item = (i32, i32)> + 'a {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+),\s+(\d+)").unwrap();
    }

    PATTERN.captures_iter(input).filter_map(|caps| {
        Some((caps.get(1)?.as_str().parse().ok()?, caps.get(2)?.as_str().parse().ok()?))
    })
}

fn manhattan_distance((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> i32 {
    abs(x1 - x0) + abs(y1 - y0)
}

pub fn part1(input: &str) -> usize {
    let coords = coords_iter(input).collect::<Vec<_>>();

    let (min_x, max_x) = coords.iter().map(|(x, _)| *x).minmax().into_option().unwrap();
    let (min_y, max_y) = coords.iter().map(|(_, y)| *y).minmax().into_option().unwrap();

    let closest_coord = |point: (i32, i32)| {
        let (first_coord, first_distance) = coords
            .iter()
            .map(|coord| (*coord, manhattan_distance(*coord, point)))
            .min_by_key(|(_, distance)| *distance)?;

        match coords
            .iter()
            .filter(|coord| **coord != first_coord)
            .map(|coord| manhattan_distance(*coord, point))
            .min()
        {
            Some(second_distance) if second_distance == first_distance => None,
            _ => Some(first_coord),
        }
    };

    let mut grid = HashMap::<(i32, i32), (i32, i32)>::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if let Some(coord) = closest_coord((x, y)) {
                grid.insert((x, y), coord);
            }
        }
    }

    coords
        .iter()
        .filter(|coord| {
            let coord = Some(**coord);

            for x in min_x..=max_x {
                if coord == closest_coord((x, min_y)) || coord == closest_coord((x, max_y)) {
                    return false;
                }
            }

            for y in min_y..=max_y {
                if coord == closest_coord((min_x, y)) || coord == closest_coord((max_x, y)) {
                    return false;
                }
            }

            true
        })
        .map(|coord| grid.iter().filter(|(_, c)| *c == coord).count())
        .max()
        .expect("No solution found!")
}

pub fn part2(input: &str, region_size: i32) -> usize {
    let coords = coords_iter(input).collect::<Vec<_>>();

    let (min_x, max_x) = coords.iter().map(|(x, _)| *x).minmax().into_option().unwrap();
    let (min_y, max_y) = coords.iter().map(|(_, y)| *y).minmax().into_option().unwrap();

    (min_x..=max_x)
        .cartesian_product(min_y..=max_y)
        .filter(|point| {
            coords.iter().map(|coord| manhattan_distance(*coord, *point)).sum::<i32>() < region_size
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n"), 17);
        assert_eq!(part1(INPUT), 5365);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n", 32), 16);
        assert_eq!(part2(INPUT, 10_000), 42513);
    }
}
