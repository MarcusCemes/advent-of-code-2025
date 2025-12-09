use std::cmp::Ordering;
use std::collections::BinaryHeap;

advent_of_code::solution!(8);

const CONNECTIONS: usize = if cfg!(test) { 10 } else { 1000 };
const NUM_BOXES: usize = if cfg!(test) { 20 } else { 1000 };
const NUM_LARGEST: usize = 3;

/// Finds the top K closest pairs and clusters them using a bounded heap strategy.
pub fn part_one(input: &str) -> Option<u64> {
    // Parse input into Structure of Arrays (SoA) for better cache locality
    let (xs, ys, zs) = parse_soa(input);
    let n = xs.len();

    let mut closest_junctions = BinaryHeap::with_capacity(CONNECTIONS + 1);

    // Iterate over all unique pairs of nodes (i, j)
    for i in 0..n {
        for j in (i + 1)..n {
            unsafe {
                // SAFETY: i and j are strictly bounded by 0..n derived from xs.len()
                let dx = *xs.get_unchecked(i) - *xs.get_unchecked(j);
                let dy = *ys.get_unchecked(i) - *ys.get_unchecked(j);
                let dz = *zs.get_unchecked(i) - *zs.get_unchecked(j);

                let dist_sq = dx * dx + dy * dy + dz * dz;

                if closest_junctions.len() < CONNECTIONS {
                    closest_junctions.push(Cable {
                        dist: dist_sq,
                        u: i,
                        v: j,
                    });
                } else {
                    // Peek at the largest distance in our 'smallest' collection
                    // unwrap_unchecked is safe because len >= CONNECTIONS > 0
                    let max_in_heap = closest_junctions.peek().unwrap_unchecked();

                    if dist_sq < max_in_heap.dist {
                        closest_junctions.pop();

                        closest_junctions.push(Cable {
                            dist: dist_sq,
                            u: i,
                            v: j,
                        });
                    }
                }
            }
        }
    }

    // Standard Union-Find to group the nodes based on the edges found
    let mut dsu = DisjointSetUnion::new(n);

    for edge in closest_junctions {
        dsu.union(edge.u, edge.v);
    }

    // Tally sizes, linear vector is faster than HashMap for small N
    let mut sizes = vec![0; n];

    for i in 0..n {
        sizes[dsu.find(i)] += 1;
    }

    let mut active_sizes: Vec<u32> = sizes.into_iter().filter(|&s| s > 0).collect();

    // Partial sort to get the top K largest sizes (active_sizes.len() >= NUM_LARGEST)
    active_sizes.select_nth_unstable_by(NUM_LARGEST, |a, b| b.cmp(a));

    let answer = active_sizes
        .iter()
        .take(NUM_LARGEST)
        .map(|&x| x as u64)
        .product();

    Some(answer)
}

/// Finds the bottleneck edge in the Minimum Spanning Tree using Prim's Algorithm.
/// Traverses `min_dists` linearly for cache efficiency, which is very cache-friendly
/// and vectorizable by the compiler compared to a PriorityQueue for dense graphs.
pub fn part_two(input: &str) -> Option<u64> {
    // Parse input into Structure of Arrays (SoA) for better cache locality
    let (xs, ys, zs) = parse_soa(input);
    let n = xs.len();

    // Distances from MST to each node
    let mut min_dists = vec![i64::MAX; n];

    // The node in the MST that connects to i
    let mut parent = vec![0; n];
    let mut visited = vec![false; n];

    unsafe { *min_dists.get_unchecked_mut(0) = 0 };

    let mut max_mst_edge_dist = 0;
    let mut answer_coords = (0, 0);

    // Add N nodes to the MST (Prim's Algorithm)
    for _ in 0..n {
        let mut u = usize::MAX;
        let mut min_val = i64::MAX;

        // Find closest unvisited node to the MST
        for i in 0..n {
            unsafe {
                let v = *visited.get_unchecked(i);
                let d = *min_dists.get_unchecked(i);

                // Branchless-friendly logic
                if !v && d < min_val {
                    min_val = d;
                    u = i;
                }
            }
        }

        unsafe { *visited.get_unchecked_mut(u) = true };

        // Check if this is the new heaviest edge in MST.
        // Skip the first node as it has no parent edge.
        if u != 0 {
            if min_val > max_mst_edge_dist {
                max_mst_edge_dist = min_val;

                unsafe {
                    let p = *parent.get_unchecked(u);
                    answer_coords = (*xs.get_unchecked(u), *xs.get_unchecked(p));
                }
            }
        }

        unsafe {
            let ux = *xs.get_unchecked(u);
            let uy = *ys.get_unchecked(u);
            let uz = *zs.get_unchecked(u);

            for v in 0..n {
                // If v is not visited, check if we found a shorter path to it via u
                if !*visited.get_unchecked(v) {
                    let dx = ux - *xs.get_unchecked(v);
                    let dy = uy - *ys.get_unchecked(v);
                    let dz = uz - *zs.get_unchecked(v);

                    let dist = dx * dx + dy * dy + dz * dz;

                    if dist < *min_dists.get_unchecked(v) {
                        *min_dists.get_unchecked_mut(v) = dist;
                        *parent.get_unchecked_mut(v) = u;
                    }
                }
            }
        }
    }

    Some((answer_coords.0 * answer_coords.1) as u64)
}

/* === Helper Structures & Functions === */

// Simple Edge struct for the Heap
#[derive(Eq, PartialEq)]
struct Cable {
    dist: i64,
    u: usize,
    v: usize,
}

impl Ord for Cable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Cable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Splits input into three separate vectors for X, Y, Z.
/// This improves SIMD vectorization potential in distance calculations.
fn parse_soa(input: &str) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut xs = Vec::with_capacity(NUM_BOXES);
    let mut ys = Vec::with_capacity(NUM_BOXES);
    let mut zs = Vec::with_capacity(NUM_BOXES);

    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Skip non-digit/minus
        while i < bytes.len() && !bytes[i].is_ascii_digit() {
            i += 1;
        }

        if i >= bytes.len() {
            break;
        }

        xs.push(parse_i64_fast(bytes, &mut i));

        while i < bytes.len() && !bytes[i].is_ascii_digit() {
            i += 1;
        }

        ys.push(parse_i64_fast(bytes, &mut i));

        while i < bytes.len() && !bytes[i].is_ascii_digit() {
            i += 1;
        }

        zs.push(parse_i64_fast(bytes, &mut i));
    }

    (xs, ys, zs)
}

fn parse_i64_fast(bytes: &[u8], i: &mut usize) -> i64 {
    let mut num = 0;
    let mut sign = 1;

    if *i < bytes.len() && bytes[*i] == b'-' {
        sign = -1;
        *i += 1;
    }

    while *i < bytes.len() && bytes[*i].is_ascii_digit() {
        num = num * 10 + (bytes[*i] - b'0') as i64;
        *i += 1;
    }

    num * sign
}

/* === Disjoint Set Union === */

struct DisjointSetUnion {
    parents: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
        }
    }

    // Path compression only (Rank/Size optimization not strictly needed for N=1000)
    fn find(&mut self, i: usize) -> usize {
        let mut root = i;
        while root != self.parents[root] {
            root = self.parents[root];
        }
        let mut curr = i;
        while curr != root {
            let next = self.parents[curr];
            self.parents[curr] = root;
            curr = next;
        }
        root
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parents[root_i] = root_j;
        }
    }
}

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
