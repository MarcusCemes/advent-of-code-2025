use std::{
    cmp::{max, min},
    ops::RangeInclusive,
    str::Lines,
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let ranges = create_disjoint_ranges(iter_ranges(&mut lines));
    let ids = iter_ids(&mut lines);

    // Count how many numbers fall within any of the given ranges
    let answer = ids.filter(|n| ranges.iter().any(|r| r.contains(n))).count() as u64;

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let ranges = create_disjoint_ranges(iter_ranges(&mut lines));

    // Now calculate the total covered space by the disjoint ranges
    let answer = ranges.iter().map(|r| r.end() - r.start() + 1).sum();

    Some(answer)
}

/// Merges overlapping ranges into disjoint ranges.
fn create_disjoint_ranges(
    ranges: impl Iterator<Item = RangeInclusive<u64>>,
) -> Vec<RangeInclusive<u64>> {
    let mut disjoint_ranges = Vec::new();

    for mut range in ranges {
        let mut cursor = 0;

        while let Some(target) = disjoint_ranges.get(cursor) {
            if are_disjoint(&range, target) {
                cursor += 1;
                continue;
            }

            // Merge the two ranges with a simple min/max
            let start = min(*range.start(), *target.start());
            let end = max(*range.end(), *target.end());
            range = start..=end;

            // Remove the merged range, but do not advance the cursor
            disjoint_ranges.swap_remove(cursor);
        }

        // Add the (possibly merged) range to the list
        disjoint_ranges.push(range);
    }

    disjoint_ranges
}

/// Returns true if the two ranges do not overlap (or touch) at all.
fn are_disjoint(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.end() < b.start() || b.end() < a.start()
}

/// Parses the input lines into ranges until an empty line is encountered.
fn iter_ranges<'a>(lines: &'a mut Lines) -> impl Iterator<Item = RangeInclusive<u64>> + 'a {
    lines.take_while(|line| !line.is_empty()).map(|line| {
        let (a, b) = line.split_once('-').unwrap();
        let start = a.parse().unwrap();
        let end = b.parse().unwrap();

        start..=end
    })
}

/// Parses the remaining input lines into IDs.
fn iter_ids(lines: &mut Lines) -> impl Iterator<Item = u64> {
    lines.map(|id| id.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
