use std::{cmp::Reverse, collections::BinaryHeap};

advent_of_code::solution!(9);

/// This tuning constant limits the number of candidates to keep
/// track of during part 2. It should be as small as possible. If
/// the best candidate is not among the top MAX_CANDIDATES by area,
/// the algorithm will return 0.
const MAX_CANDIDATES: usize = 65536;

const NUMBER_POINTS: usize = 496;

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let answer = solve_part_one(points);

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let edges = Edges::build(&points);
    let answer = solve_part_two(points, edges);

    Some(answer)
}

/* === Parsing === */

/// Simple structure of arrays for points
struct Points {
    x: Box<[i32]>,
    y: Box<[i32]>,
}

fn parse_input(input: &str) -> Points {
    let mut x = Vec::with_capacity(NUMBER_POINTS);
    let mut y = Vec::with_capacity(NUMBER_POINTS);

    for line in input.lines() {
        let (a, b) = line.split_once(',').unwrap();

        x.push(a.parse().unwrap());
        y.push(b.parse().unwrap());
    }

    Points {
        x: x.into_boxed_slice(),
        y: y.into_boxed_slice(),
    }
}

/* === Part 1 === */

fn solve_part_one(points: Points) -> u64 {
    let mut answer = 0;

    // Optimises away bound checks in the loop without unsafe code (small cost)
    let n = points.x.len().min(points.y.len());

    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points.x[i].abs_diff(points.x[j]) as u64 + 1;
            let dy = points.y[i].abs_diff(points.y[j]) as u64 + 1;

            answer = answer.max(dx * dy);
        }
    }

    answer
}

/* === Part 2 === */

fn solve_part_two(points: Points, edges: Edges) -> u64 {
    // Optimises away bound checks in the loop without unsafe code (small cost)
    let n = points.x.len().min(points.y.len());

    // Use a min-heap of size MAX_CANDIDATES to keep top candidates
    let mut heap = BinaryHeap::<Reverse<(u64, usize, usize)>>::with_capacity(MAX_CANDIDATES);

    // Similar to part one, but we keep a bounded heap of largest areas
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points.x[i].abs_diff(points.x[j]) as u64 + 1;
            let dy = points.y[i].abs_diff(points.y[j]) as u64 + 1;
            let area = dx * dy;

            if heap.len() < MAX_CANDIDATES {
                heap.push(Reverse((area, i, j)));
            } else if let Some(&Reverse((min_area, _, _))) = heap.peek() {
                if area > min_area {
                    heap.pop();
                    heap.push(Reverse((area, i, j)));
                }
            }
        }
    }

    // Drain the heap into a vector...
    let mut candidates: Vec<_> = heap.into_iter().map(|Reverse(x)| x).collect();

    // ...and sort by area descending
    candidates.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    for (area, i, j) in candidates {
        let (&x1, &x2) = unsafe { (points.x.get_unchecked(i), points.x.get_unchecked(j)) };
        let (&y1, &y2) = unsafe { (points.y.get_unchecked(i), points.y.get_unchecked(j)) };

        // mid_x * 2 = x1 + x2, mid_y * 2 = y1 + y2 (scaled coordinates)
        let mid_x2 = x1 + x2;
        let mid_y2 = y1 + y2;

        // 1. Check if midpoint is inside polygon
        if !is_point_in_polygon(mid_x2, mid_y2, &edges.vertical) {
            continue;
        }

        let x = (x1.min(x2), x1.max(x2));
        let y = (y1.min(y2), y1.max(y2));

        // 2. Check if rectangle edges intersect polygon interior
        if !edges_intersect_interior(x, y, &edges) {
            return area;
        }
    }

    0
}

/* -- Edge pre-computation -- */

struct Edges {
    horizontal: Box<[Edge]>,
    vertical: Box<[Edge]>,
}

struct Edge {
    at: i32,
    min: i32,
    max: i32,
}

impl Edges {
    /// Build precomputed edge structures, separating them by orientation and
    /// sorting for efficient querying.
    fn build(points: &Points) -> Edges {
        let n = points.x.len().min(points.y.len());

        let mut vertical = Vec::with_capacity(n / 2);
        let mut horizontal = Vec::with_capacity(n / 2);

        for i in 0..n {
            let j = (i + 1) % n;

            let (x1, y1) = (points.x[i], points.y[i]);
            let (x2, y2) = (points.x[j], points.y[j]);

            if x1 == x2 {
                let (min, max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                vertical.push(Edge { at: x1, min, max });
            } else {
                let (min, max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                horizontal.push(Edge { at: y1, min, max });
            }
        }

        vertical.sort_unstable_by_key(|e| e.at);
        horizontal.sort_unstable_by_key(|e| e.at);

        Edges {
            vertical: vertical.into_boxed_slice(),
            horizontal: horizontal.into_boxed_slice(),
        }
    }
}

/// Point-in-polygon using integer arithmetic (2x scaled coordinates).
/// Uses ray casting with vertical edges only (horizontal edges don't affect vertical ray).
fn is_point_in_polygon(px2: i32, py2: i32, v_edges: &[Edge]) -> bool {
    let mut inside = false;

    for edge in v_edges {
        // Edge x coordinate in 2x scale
        let edge_x2 = edge.at * 2;

        // Check if edge is to the right of point
        if edge_x2 <= px2 {
            continue;
        }

        // Check if point's y (in 2x scale) is strictly between edge's y range (in 2x scale)
        let edge_min_y2 = edge.min * 2;
        let edge_max_y2 = edge.max * 2;

        if py2 > edge_min_y2 && py2 < edge_max_y2 {
            inside = !inside;
        }
    }

    inside
}

/// Check if any edge intersects the interior of the rectangle
fn edges_intersect_interior(x: (i32, i32), y: (i32, i32), edges: &Edges) -> bool {
    let (min_x, max_x) = x;
    let (min_y, max_y) = y;

    // Binary search to find vertical edges with x in (min_x, max_x)
    let v_start = edges.vertical.partition_point(|e| e.at <= min_x);
    let v_end = edges.vertical.partition_point(|e| e.at < max_x);

    for edge in &edges.vertical[v_start..v_end] {
        if edge.min.max(min_y) < edge.max.min(max_y) {
            return true;
        }
    }

    // Binary search to find horizontal edges with y in (min_y, max_y)
    let h_start = edges.horizontal.partition_point(|e| e.at <= min_y);
    let h_end = edges.horizontal.partition_point(|e| e.at < max_y);

    for edge in &edges.horizontal[h_start..h_end] {
        if edge.min.max(min_x) < edge.max.min(max_x) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
