use std::collections::BTreeMap;

use glam::I64Vec3;
use itertools::Itertools;

advent_of_code::solution!(8);

type MapImpl<K, V> = BTreeMap<K, V>;
type Vector = I64Vec3;

#[cfg(test)]
const CONNECTIONS: usize = 10;

#[cfg(not(test))]
const CONNECTIONS: usize = 1000;

const NUM_LARGEST: usize = 3;

pub fn part_one(input: &str) -> Option<u64> {
    let (junction_boxes, pairs, mut dsu) = generate_pairs(input);

    // Connect the closest CONNECTIONS pairs
    for (_, a, b) in pairs.into_iter().take(CONNECTIONS) {
        dsu.union(a, b);
    }

    let mut sizes_map = MapImpl::new();

    // Iterate through all junction boxes and increment size of the root
    for i in 0..junction_boxes.len() {
        *sizes_map.entry(dsu.find(i)).or_insert(0) += 1;
    }

    // Collect and sort the sizes of each connected component
    let mut sizes = sizes_map.values().cloned().collect::<Vec<_>>();
    sizes.sort_unstable();

    // Take the product of NUM_LARGEST largest sizes
    let answer = sizes.iter().rev().take(NUM_LARGEST).product();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (junction_boxes, pairs, mut dsu) = generate_pairs(input);

    // Keep connecting the closest pairs...
    for (_, a, b) in pairs {
        dsu.union(a, b);

        // until all junction boxes are connected,
        // returning the product of their x-coordinates
        if dsu.num_sets == 1 {
            return Some((junction_boxes[a].x * junction_boxes[b].x) as u64);
        }
    }

    None
}

/* === Input parsing === */

/// Generates the list of nodes, ordered pairs by distance, and a new
/// Disjoint Set Union object for the given set of nodes.
fn generate_pairs(input: &str) -> (Vec<Vector>, Vec<(i64, usize, usize)>, DisjointSetUnion) {
    let nodes = parse_input(input).collect::<Vec<_>>();

    // Generate all unique pairs of nodes with their squared distances
    let mut pairs = (0..nodes.len())
        .tuple_combinations()
        .map(|(a, b)| (nodes[a].distance_squared(nodes[b]), a, b))
        .collect::<Vec<_>>();

    // Sort pairs by distance (ascending)
    pairs.sort_unstable_by_key(|(d, _, _)| *d);

    let dsu = DisjointSetUnion::new(nodes.len());

    (nodes, pairs, dsu)
}

fn parse_input(input: &str) -> impl Iterator<Item = Vector> {
    input.lines().map(|line| {
        let (x, rest) = line.split_once(",").unwrap();
        let (y, z) = rest.split_once(",").unwrap();

        Vector::from_array([x, y, z].map(|c| c.parse().unwrap()))
    })
}

/* === Disjoint Set Union === */

/// A simple Disjoint Set Union (Union-Find) data structure implementation.
/// Used to efficiently manage and merge sets of elements, such as connected
/// components in a graph-colouring problem.
struct DisjointSetUnion {
    num_sets: usize,
    parents: Vec<usize>,
}

impl DisjointSetUnion {
    /// Creates a new Disjoint Set Union with `size` elements.
    fn new(size: usize) -> Self {
        Self {
            num_sets: size,
            parents: (0..size).collect(),
        }
    }

    /// Finds the root of the set containing element `i`.
    fn find(&mut self, mut i: usize) -> usize {
        while i != self.parents[i] {
            self.parents[i] = self.parents[self.parents[i]];
            i = self.parents[i];
        }

        i
    }

    /// Unites the sets containing elements `i` and `j`.
    /// Returns `true` if the sets were separate and have been united, `false` otherwise.
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            self.parents[root_i] = root_j;
            self.num_sets -= 1;
            return true;
        }

        false
    }
}

/* === Tests === */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
