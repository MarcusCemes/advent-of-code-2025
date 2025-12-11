use std::collections::HashMap;

use pathfinding::prelude::count_paths;

advent_of_code::solution!(11);

type Id = u16;

const START: Id = node_id("you");
const END: Id = node_id("out");

const SVR: Id = node_id("svr");
const DAC: Id = node_id("dac");
const FFT: Id = node_id("fft");

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let n_paths = n_paths(START, END, &map);
    Some(n_paths as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);

    // Count paths: svr → dac → fft → out
    let n_paths_a = n_paths(SVR, DAC, &map) * n_paths(DAC, FFT, &map) * n_paths(FFT, END, &map);

    // Count paths: svr → fft → dac → out
    let n_paths_b = n_paths(SVR, FFT, &map) * n_paths(FFT, DAC, &map) * n_paths(DAC, END, &map);

    Some(n_paths_a + n_paths_b)
}

/* === Input === */

fn parse_input(input: &str) -> HashMap<Id, Box<[Id]>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();

        let from = node_id(&parts.next().unwrap()[..3]);
        let to = parts.map(node_id).collect::<Vec<_>>().into_boxed_slice();

        map.insert(from, to);
    }

    map
}

/* === Solver === */

fn n_paths(from: Id, to: Id, map: &HashMap<Id, Box<[Id]>>) -> u64 {
    count_paths(
        from,
        |n| map.get(n).into_iter().flatten().copied(),
        |&n| n == to,
    ) as u64
}

/* === Helpers === */

const fn node_id(node: &str) -> Id {
    const fn id_recursive(node: &[u8], value: Id) -> Id {
        match node.split_first() {
            Some((char, rest)) => id_recursive(rest, value << 5 | ((*char - b'a') as Id)),
            _ => value,
        }
    }

    id_recursive(node.as_bytes(), 0)
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
