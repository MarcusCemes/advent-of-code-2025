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

    // We can't mutate a vector while iterating over it, so we store
    // the indices of ranges that should be merged here.
    let mut overlapping_indices = Vec::new();

    for mut range in ranges {
        // Find all ranges that overlap and should therefore be joined
        for (i, other) in disjoint_ranges.iter().enumerate() {
            if !are_disjoint(&range, &other) {
                overlapping_indices.push(i);
            }
        }

        // If there are no overlapping ranges, we can just add this one
        if overlapping_indices.is_empty() {
            disjoint_ranges.push(range);
            continue;
        }

        // Merge all overlapping ranges into the new one with a simple min/max
        for other_idx in &overlapping_indices {
            // SAFETY: We know that other_idx is a valid index into overlapping_indices
            let other = unsafe { disjoint_ranges.get_unchecked(*other_idx) };

            let start = min(*range.start(), *other.start());
            let end = max(*range.end(), *other.end());

            range = start..=end;
        }

        // Remove the overlapping ranges IN REVERSE ORDER to avoid shifting indices
        for other_idx in overlapping_indices.iter().rev() {
            disjoint_ranges.swap_remove(*other_idx);
        }

        // Finally, add the merged range and clear the list for the next iteration
        disjoint_ranges.push(range);
        overlapping_indices.clear();
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
