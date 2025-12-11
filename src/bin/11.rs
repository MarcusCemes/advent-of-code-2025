use std::collections::HashMap;

use pathfinding::prelude::count_paths;

advent_of_code::solution!(11);

type Id = u16;

pub fn part_one(input: &str) -> Option<u64> {
    let graph = Graph::parse(input);
    let n_paths = graph.paths("you", "out");
    Some(n_paths as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = Graph::parse(input);

    // Count paths: svr → dac → fft → out
    let n_paths_a =
        graph.paths("svr", "dac") * graph.paths("dac", "fft") * graph.paths("fft", "out");

    // Count paths: svr → fft → dac → out
    let n_paths_b =
        graph.paths("svr", "fft") * graph.paths("fft", "dac") * graph.paths("dac", "out");

    Some(n_paths_a + n_paths_b)
}

/* === Input === */

/// Representation of the graph using an atom table and
/// a flat edge list with indices.
struct Graph<'a> {
    atoms: AtomTable<'a>,
    edges: Vec<Id>,
    indices: Vec<(usize, usize)>,
}

impl<'a> Graph<'a> {
    fn parse(input: &'a str) -> Self {
        let mut atoms = AtomTable::default();
        let mut edges = Vec::new();
        let mut indices = Vec::new();

        for line in input.lines() {
            let mut parts = line.split_ascii_whitespace();

            let from_str = parts.next().unwrap();
            let from = atoms.insert(&from_str[..3]);

            // If a node is encountered before it's definition,
            // the vector needs to be resized/padded.
            if indices.len() <= from as usize {
                indices.resize(from as usize + 1, (0, 0));
            }

            let index = edges.len();
            let mut count = 0;

            // Dump connections into a flat edge list
            for part in parts {
                edges.push(atoms.insert(part));
                count += 1;
            }

            indices[from as usize] = (index, count);
        }

        // Ensure all nodes have an index entry
        if indices.len() < atoms.next_id as usize {
            indices.resize(atoms.next_id as usize, (0, 0));
        }

        Graph {
            atoms,
            edges,
            indices,
        }
    }

    fn paths(&self, from: &str, to: &str) -> u64 {
        let from = self.atoms.get(from);
        let to = self.atoms.get(to);

        count_paths(
            from,
            |&n| unsafe {
                let &(index, count) = self.indices.get_unchecked(n as usize);

                self.edges
                    .get_unchecked(index..index + count)
                    .iter()
                    .copied()
            },
            |&n| n == to,
        ) as u64
    }
}

/* === Helpers === */

#[derive(Default)]
struct AtomTable<'a> {
    next_id: Id,
    table: HashMap<&'a str, Id>,
}

impl<'a> AtomTable<'a> {
    fn get(&self, atom: &str) -> Id {
        *self.table.get(atom).unwrap()
    }

    fn insert(&mut self, atom: &'a str) -> Id {
        match self.table.get(atom) {
            Some(&id) => id,

            None => {
                let id = self.next_id;
                self.table.insert(atom, id);
                self.next_id += 1;
                id
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    // Part 2 uses a different example
}
