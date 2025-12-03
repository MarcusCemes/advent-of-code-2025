advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    solve::<2>(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve::<12>(input)
}

/// Finds the largest number that can be formed by selecting `N` digits
/// from each line while maintaining their original order, then sums the results.
///
/// The algorithm works by selecting the maximum digit from a sliding window of
/// the input slice, ensuring that enough characters remain to complete the number.
///
/// The algorithm runs in `O(n * N)` time per line, where n is the line length.
fn solve<const N: usize>(input: &str) -> Option<u64> {
    let mut total = 0;

    for line in input.lines() {
        // It's slightly easier to work with raw bytes of the string slice rather
        // than characters, especially since we don't need UTF-8 support. The string
        // only contains ASCII digits (0x30..0x39), we can safely work with these values
        // without overflow and decode later by subtracting b'0'.
        let bytes = line.as_bytes();

        // Keep track of the accumulated number and our sliding cursor position
        let mut acc = 0;
        let mut cursor = 0;

        for i in (0..N).rev() {
            // Calculate the slice window we can inspect, while leaving enough
            // characters remaining to complete the number
            let end_offset = bytes.len() - i;
            let slice = &bytes[cursor..end_offset];

            let (mut j, mut max) = (0, 0);

            // Find the *first* occurrence of the max digit
            for (idx, &v) in slice.iter().enumerate() {
                if v > max {
                    max = v;
                    j = idx;
                }
            }

            // Append the digit to our accumulator (ASCII digit decoding)
            acc = 10 * acc + (max - b'0') as u64;

            // Move the cursor past the selected digit
            cursor += j as usize + 1;
        }

        total += acc;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
