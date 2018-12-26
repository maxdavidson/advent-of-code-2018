use core::cmp::min;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Acre {
  OpenGround,
  Trees,
  Lumberyard,
}

fn parse(input: &str) -> (Vec<Acre>, usize) {
  let acres = input
    .chars()
    .filter_map(|c| match c {
      '.' => Some(Acre::OpenGround),
      '|' => Some(Acre::Trees),
      '#' => Some(Acre::Lumberyard),
      _ => None,
    })
    .collect::<Vec<_>>();

  // Integer square root...
  let size = (1..acres.len()).find(|i| i * i == acres.len()).expect("Invalid size");

  (acres, size)
}

#[allow(dead_code)]
fn draw(acres: &[Acre], size: usize) {
  for y in 0..size {
    let line = (0..size)
      .map(|x| match acres[x + size * y] {
        Acre::OpenGround => '.',
        Acre::Trees => '|',
        Acre::Lumberyard => '#',
      })
      .collect::<String>();
    println!("{}", line);
  }
}

#[allow(dead_code)]
fn solve(input: &str, iterations: usize) -> usize {
  let (acres, size) = parse(input);

  let mut index_by_hash = HashMap::<u64, usize>::new();
  let mut answers_by_index = Vec::<usize>::new();

  let mut next_acres = acres.clone();
  let mut prev_acres = acres;

  let mut i = 0;

  loop {
    for y in 0..size {
      for x in 0..size {
        let mut trees_count = 0;
        let mut lumberyard_count = 0;

        let mut count = |x, y| match prev_acres[x + size * y] {
          Acre::Trees => trees_count += 1,
          Acre::Lumberyard => lumberyard_count += 1,
          _ => {}
        };

        #[allow(clippy::needless_range_loop)]
        for y2 in if y == 0 { 0 } else { y - 1 }..min(y + 2, size) {
          for x2 in if x == 0 { 0 } else { x - 1 }..min(x + 2, size) {
            if (x, y) != (x2, y2) {
              count(x2, y2);
            }
          }
        }

        next_acres[x + size * y] = match prev_acres[x + size * y] {
          Acre::OpenGround if trees_count >= 3 => Acre::Trees,
          Acre::Trees if lumberyard_count >= 3 => Acre::Lumberyard,
          Acre::Lumberyard if trees_count == 0 || lumberyard_count == 0 => Acre::OpenGround,
          acre => acre,
        };
      }
    }

    let mut trees_count = 0;
    let mut lumberyard_count = 0;

    for acre in next_acres.iter() {
      match acre {
        Acre::Trees => trees_count += 1,
        Acre::Lumberyard => lumberyard_count += 1,
        _ => {}
      }
    }

    let answer = trees_count * lumberyard_count;

    if i + 1 == iterations {
      return answer;
    }

    let hash = {
      let mut hasher = DefaultHasher::new();
      Hash::hash_slice(&next_acres, &mut hasher);
      hasher.finish()
    };

    if let Some(i0) = index_by_hash.get(&hash) {
      let repeating_slice = &answers_by_index[*i0..i];
      return repeating_slice[(iterations - i0 - 1) % repeating_slice.len()];
    }

    answers_by_index.push(answer);
    index_by_hash.insert(hash, i);
    prev_acres.clone_from(&next_acres);

    i += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = include_str!("test_input");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(solve(TEST_INPUT, 10), 1147);
    assert_eq!(solve(INPUT, 10), 511_000);
  }

  #[test]
  fn part2_works() {
    assert_eq!(solve(INPUT, 542), 211_050);
    assert_eq!(solve(INPUT, 1_000_000_000), 194_934);
  }
}
