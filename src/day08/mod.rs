use core::str::FromStr;
use num::Unsigned;

fn nums_iter<'a, T: FromStr + Unsigned>(input: &'a str) -> impl Iterator<Item = T> + 'a {
    input.split_whitespace().filter_map(|val| val.parse().ok())
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn from_slice_helper(slice: &[usize]) -> (Node, &[usize]) {
        let (header_slice, mut tail_slice) = slice.split_at(2);
        let children_count = header_slice[0];
        let metadata_count = header_slice[1];

        let mut children = Vec::with_capacity(children_count);
        let mut metadata = Vec::with_capacity(metadata_count);

        for _ in 0..children_count {
            let (child_node, child_tail_slice) = Node::from_slice_helper(tail_slice);
            children.push(child_node);
            tail_slice = child_tail_slice;
        }

        let (metadata_slice, tail_slice) = tail_slice.split_at(metadata_count);

        for metadata_value in metadata_slice {
            metadata.push(*metadata_value);
        }

        (Node { children, metadata }, tail_slice)
    }

    fn from_slice(slice: &[usize]) -> Node {
        let (node, _) = Node::from_slice_helper(slice);
        node
    }
}

impl From<&[usize]> for Node {
    fn from(slice: &[usize]) -> Node {
        Node::from_slice(slice)
    }
}

impl From<&str> for Node {
    fn from(input: &str) -> Node {
        let data = nums_iter(input).collect::<Vec<_>>();
        Node::from_slice(&data)
    }
}

fn metadata_sum(Node { children, metadata }: &Node) -> usize {
    children.iter().map(metadata_sum).sum::<usize>() + metadata.iter().sum::<usize>()
}

fn node_value(Node { children, metadata }: &Node) -> usize {
    if children.is_empty() {
        metadata.iter().sum()
    } else {
        metadata.iter().filter_map(|i| children.get(*i - 1)).map(node_value).sum()
    }
}

pub fn part1(input: &str) -> usize {
    let root_node = Node::from(input);
    metadata_sum(&root_node)
}

pub fn part2(input: &str) -> usize {
    let root_node = Node::from(input);
    node_value(&root_node)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
        assert_eq!(part1(INPUT), 49602);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 66);
        assert_eq!(part2(INPUT), 25656);
    }
}
