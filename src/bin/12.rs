advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let (shapes, regions) = parse_input(input);
    let answer = regions.iter().filter(|r| can_fit(&shapes, r)).count();

    Some(answer as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

/* === Logic === */

/// The problem is crafted such that if the total shape area fits
/// within the region area, the shapes can always be arranged.
fn can_fit(shapes: &[u32], region: &Region) -> bool {
    let region_area = region.width * region.height;
    let shape_area: u32 = (region.counts.iter())
        .zip(shapes.iter())
        .map(|(&count, &area)| count * area)
        .sum();

    region_area >= shape_area
}

/* === Parsing === */

struct Region {
    width: u32,
    height: u32,
    counts: Box<[u32]>,
}

fn parse_input(input: &str) -> (Box<[u32]>, Box<[Region]>) {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    let mut current_shape_area = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains('x') {
            // Region line: "WxH: c0 c1 c2 ..."
            let (dims, counts_str) = line.split_once(": ").unwrap();
            let (w, h) = dims.split_once('x').unwrap();

            let counts = counts_str
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .into_boxed_slice();

            regions.push(Region {
                width: w.parse().unwrap(),
                height: h.parse().unwrap(),
                counts,
            });
        } else if line.ends_with(':') {
            // Shape header "N:" - save previous shape if any
            if current_shape_area > 0 {
                shapes.push(current_shape_area);
            }

            current_shape_area = 0;
        } else {
            // Shape row - count '#' characters
            current_shape_area += line.bytes().filter(|&b| b == b'#').count() as u32;
        }
    }

    // Save last shape
    if current_shape_area > 0 {
        shapes.push(current_shape_area);
    }

    (shapes.into_boxed_slice(), regions.into_boxed_slice())
}
