use std::fmt::Debug;

use aoc_lib::*;
use petgraph::{graph::NodeIndex, Graph, Undirected};
use tinyvec::{array_vec, ArrayVec};

aoc_setup!(Day12, sample 1: 10, sample 2: 36, part 1: 3497, part 2: 93686);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Node(u16);

impl Node {
    pub const START: Self = Node(0);
    pub const END: Self = Node(1);

    pub fn parse(label: &str) -> Self {
        match label {
            "start" => Self::START,
            "end" => Self::END,
            label if label.len() == 2 => {
                Node(u16::from_le_bytes(label.as_bytes().try_into().unwrap()))
            }
            label if label.len() == 1 => Self::parse(format!("{}{}", label, label).as_str()),
            _ => panic!("Invalid node label: {}", label),
        }
    }

    pub const fn can_visit_multiple_times(&self) -> bool {
        matches!((self.0 & 0xFF) as u8, b'A'..=b'Z')
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_tuple("Node").field(&self.0).finish()
        match *self {
            Node::START => write!(f, "start"),
            Node::END => write!(f, "end"),
            _ => {
                let chars = [self.0 as u8, (self.0 >> 8) as u8];
                let chars = [chars[0] as char, chars[1] as char];
                f.write_str(&format!("{}{}", chars[0], chars[1]))
            }
        }
    }
}

impl From<NodeIndex> for Node {
    fn from(idx: NodeIndex) -> Self {
        Node(idx.index() as u16)
    }
}

impl From<Node> for NodeIndex {
    fn from(node: Node) -> Self {
        Self::new(node.0 as usize)
    }
}

pub struct Day12;

impl AdventOfCode for Day12 {
    type Input = Graph<Node, (), Undirected>;
    type Output = usize;

    fn parse_input(s: &str) -> Self::Input {
        Self::Input::from_edges(s.lines().map(|line| {
            let (a, b) = line.split_once("-").unwrap();
            (Node::parse(a), Node::parse(b))
        }))
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        walk_edge_1(input, array_vec!(Node::START.into()))
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        walk_edge_2(input, array_vec!(Node::START.into()), false)
    }
}

fn walk_edge_1(graph: &Graph<Node, (), Undirected>, path: ArrayVec<[NodeIndex; 16]>) -> usize {
    let last_node = *path.last().unwrap();
    let neighbours = graph
        .neighbors(last_node)
        .filter(|&n| n != last_node)
        .filter(|&n| Node::from(n).can_visit_multiple_times() || !path.contains(&n));

    neighbours.fold(0, |acc, neighbour| {
        let mut new_path = path.clone(); // TODO: dont need to clone if I properly dequeue
        new_path.push(neighbour);

        acc + if neighbour == Node::END.into() {
            // println!("{:?}", new_path);
            1
        } else {
            walk_edge_1(graph, new_path)
        }
    })
}

fn walk_edge_2(
    graph: &Graph<Node, (), Undirected>,
    path: ArrayVec<[Node; 16]>,
    duplicate_used: bool,
) -> usize {
    let last_node = *path.last().unwrap();
    let neighbours = graph
        .neighbors(last_node.into())
        .map(|n| n.into())
        .filter(|&n| n != last_node && n != Node::START)
        .filter(|&n| {
            Node::from(n).can_visit_multiple_times() || !duplicate_used || !path.contains(&n)
        });

    neighbours.fold(0, |acc, neighbour| {
        let duplicate_used =
            duplicate_used || (!neighbour.can_visit_multiple_times() && path.contains(&neighbour));

        let mut new_path = path.clone(); // TODO: dont need to clone if I properly dequeue
        new_path.push(neighbour);

        acc + if neighbour == Node::END.into() {
            // println!("{:?}", new_path);
            1
        } else {
            walk_edge_2(graph, new_path, duplicate_used)
        }
    })
}
