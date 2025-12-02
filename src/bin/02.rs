use std::ops::RangeInclusive;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let answer = parse_input(input).flat_map(invalid_ids).sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let answer = parse_input(input).flat_map(iter_range_invalid_ids).sum();
    Some(answer)
}

fn parse_input(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input.split(',').map(|range| {
        let (a, b) = range.split_once('-').unwrap();
        a.trim().parse().unwrap()..=b.trim().parse().unwrap()
    })
}

/* === Part 1 === */

/// A dumb implementation that enumerates the entire range and checks
/// whether each ID is composed of two identical halves.
fn invalid_ids(range: RangeInclusive<u64>) -> impl Iterator<Item = u64> {
    range.flat_map(|id| {
        let n = digits(id);

        if n % 2 == 1 {
            return None;
        }

        let first_half = id % 10u64.pow(n / 2);
        let second_half = id / 10u64.pow(n / 2);

        (first_half == second_half).then_some(id)
    })
}

/* === Part 2 === */

/// A slightly smarter implementation that generates all possible repeatable
/// blocks for the digit counts in the given range, and filters them to those
/// that fall within the range.
fn iter_range_invalid_ids(range: RangeInclusive<u64>) -> impl Iterator<Item = u64> {
    let start = *range.start();
    let end = *range.end();

    let start_digits = digits(start);
    let end_digits = digits(end);

    // For each possible number of digits in the range...
    (start_digits..=end_digits).flat_map(move |digits| {
        // Generate all invalid IDs with that number of digits
        iter_invalid_ids(digits)
            // Filter to only those within the given range
            .filter(move |id| *id >= start && *id <= end)
    })
}

/// Returns an iterator over all invalid IDs with the given number of digits.
fn iter_invalid_ids(digits: u32) -> impl Iterator<Item = u64> {
    // For each possible block length that can evenly divide digits...
    repeatable_lengths(digits).flat_map(move |block_length| {
        // Compute the number of blocks and the start/end repeated block values
        let blocks = digits / block_length;
        let start = 10u64.pow(block_length - 1);
        let end = repeat_block(9, block_length, 1);

        // For each possible repeated block value...
        (start..=end)
            // Generate the full repeated ID...
            .map(move |block| repeat_block(block, blocks, block_length))
            // Filter out those that have smaller repeated blocks within them,
            // for example, 2222 has repeated blocks "2" (4 times) and "22" (2 times)
            .filter(move |n| {
                repeatable_lengths(digits)
                    .filter(|l| *l < block_length)
                    .all(|length| !is_repeated_block(*n, length))
            })
    })
}

/// Checks if n is made up of repeated blocks of the given length.
fn is_repeated_block(n: u64, block_length: u32) -> bool {
    let chunk = n % 10u64.pow(block_length);
    let repeated = repeat_block(chunk, digits(n) / block_length, block_length);
    n == repeated
}

/// Returns an iterator over all block lengths that can evenly divide n.
fn repeatable_lengths(n: u32) -> impl Iterator<Item = u32> {
    (1..=n / 2).filter(move |block_length| n % block_length == 0)
}

/// Repeats the given block a number of times to form a new number.
fn repeat_block(mut block: u64, blocks: u32, block_length: u32) -> u64 {
    let block_multiplier = 10u64.pow(block_length);

    (0..blocks).fold(0, |acc, _| {
        let new_acc = acc + block;
        block *= block_multiplier;
        new_acc
    })
}

/// Returns the number of digits in n.
fn digits(n: u64) -> u32 {
    n.ilog10() + 1
}

/* === Tests === */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_repeat_block() {
        assert_eq!(repeat_block(12, 3, 2), 121212);
        assert_eq!(repeat_block(5, 4, 1), 5555);
        assert_eq!(repeat_block(15, 1, 3), 15);
    }
}
