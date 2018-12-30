use core::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Position = (usize, usize);

#[derive(Debug, Copy, Clone)]
enum Region {
  Rocky,
  Wet,
  Narrow,
}

impl Region {
  fn compatible_tools(self) -> &'static [Tool] {
    match self {
      Region::Rocky => &[Tool::ClimbingGear, Tool::Torch],
      Region::Wet => &[Tool::ClimbingGear, Tool::Neither],
      Region::Narrow => &[Tool::Torch, Tool::Neither],
    }
  }
}

impl Region {
  fn from_erosion_level(erosion_level: usize) -> Region {
    match erosion_level % 3 {
      0 => Region::Rocky,
      1 => Region::Wet,
      2 => Region::Narrow,
      _ => panic!("This shouldn't happend!"),
    }
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tool {
  ClimbingGear,
  Torch,
  Neither,
}

fn compute_erosion_levels(
  depth: usize,
  target: Position,
  x_scale: usize,
  y_scale: usize,
) -> HashMap<Position, usize> {
  let mut erosion_levels = HashMap::new();

  for x in 0..=target.0 * x_scale {
    for y in 0..=target.1 * y_scale {
      let geologic_index = match (x, y) {
        (0, 0) => 0,
        (_, 0) => x * 16807,
        (0, _) => y * 48271,
        pos if pos == target => 0,
        (_, _) => erosion_levels[&(x - 1, y)] * erosion_levels[&(x, y - 1)],
      };

      erosion_levels.insert((x, y), (geologic_index + depth) % 20183);
    }
  }

  erosion_levels
}

fn manhattan_distance((x0, y0): Position, (x1, y1): Position) -> usize {
  let dx = if x0 < x1 { x1 - x0 } else { x0 - x1 };
  let dy = if y0 < y1 { y1 - y0 } else { y0 - y1 };
  dx + dy
}

fn neighbor_positions((x, y): Position) -> [Option<Position>; 4] {
  [
    if x > 0 { Some((x - 1, y)) } else { None },
    if y > 0 { Some((x, y - 1)) } else { None },
    Some((x + 1, y)),
    Some((x, y + 1)),
  ]
}

// Compute the shortest path from (0, 0) to the target
#[allow(dead_code)]
fn compute_shortest_distance(
  target: Position,
  regions: &HashMap<Position, Region>,
) -> Option<usize> {
  #[derive(Clone, Eq, PartialEq, Debug)]
  struct Node {
    total_distance: usize,
    equipped_tool: Tool,
    position: Position,
  }

  impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
      self.total_distance.cmp(&other.total_distance).reverse()
    }
  }

  impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(&other))
    }
  }

  const MOVEMENT_COST: usize = 1;
  const TOOL_SWITCHING_COST: usize = 7;

  let start_position = (0, 0);
  let mut distances = HashMap::new();
  let mut priority_queue = BinaryHeap::new();

  distances.insert(((0, 0), Tool::Torch), 0usize);

  priority_queue.push(Node {
    position: start_position,
    equipped_tool: Tool::Torch,
    total_distance: manhattan_distance(start_position, target),
  });

  while let Some(Node { position, equipped_tool, total_distance }) = priority_queue.pop() {
    if position == target && equipped_tool == Tool::Torch {
      return Some(total_distance);
    }

    let region = regions[&position];
    let distance = distances[&(position, equipped_tool)];

    let mut enqueue_node = |position, distance, tool| match distances.get(&(position, tool)) {
      Some(best_distance) if *best_distance <= distance => {}
      _ => {
        distances.insert((position, tool), distance);
        priority_queue.push(Node {
          position,
          equipped_tool: tool,
          total_distance: distance + manhattan_distance(position, target),
        });
      }
    };

    for tool in region.compatible_tools() {
      enqueue_node(position, distance + TOOL_SWITCHING_COST, *tool);
    }

    for neighbor_position in neighbor_positions(position).iter().flatten() {
      if let Some(neighbor_region) = regions.get(&neighbor_position) {
        if neighbor_region.compatible_tools().contains(&equipped_tool) {
          enqueue_node(*neighbor_position, distance + MOVEMENT_COST, equipped_tool);
        }
      }
    }
  }

  distances.get(&(target, Tool::Torch)).cloned()
}

pub fn part1(depth: usize, target: Position) -> usize {
  compute_erosion_levels(depth, target, 1, 1).values().map(|erosion_level| erosion_level % 3).sum()
}

pub fn part2(depth: usize, target: Position) -> usize {
  let regions: HashMap<Position, Region> = compute_erosion_levels(depth, target, 5, 2)
    .into_iter()
    .map(|(position, erosion_level)| (position, Region::from_erosion_level(erosion_level)))
    .collect();

  compute_shortest_distance(target, &regions).expect("There wasn't a shortest path")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_works() {
    assert_eq!(part1(510, (10, 10)), 114);
    assert_eq!(part1(3339, (10, 715)), 7915);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(510, (10, 10)), 45);
    assert_eq!(part2(3339, (10, 715)), 980);
  }
}
