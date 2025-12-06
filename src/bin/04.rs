use std::{iter, mem};

use itertools::{Itertools, izip};

advent_of_code::solution!(4);

/// Counts the number of removable rolls in a single pass of the input string
/// with three synchronized sliding windows. Does not allocate and runs in `O(n)`
/// time, where n is the total input size.
pub fn part_one(input: &str) -> Option<u64> {
    let bytes = input.as_bytes();
    let line_length = bytes.iter().position(|&b| b == b'\n')?;
    let iter_bytes = bytes.iter().copied();

    let mut answer = 0;

    // Pad window by 1 at start so first center (e) is at index 0
    let window = iter::once(b'\n')
        .chain(iter_bytes.clone())
        .tuple_windows::<(u8, u8, u8)>();

    // Previous row: pad by line_length + 1 (one full row) + 1 (for window padding) = line_length + 2
    let previous = std::iter::repeat_n(b'\n', line_length + 2)
        .chain(iter_bytes.clone())
        .tuple_windows::<(u8, u8, u8)>();

    // Next row: skip line_length (not +1, since window is padded by 1)
    let next = iter_bytes
        .skip(line_length)
        .chain(iter::repeat(b'\n'))
        .tuple_windows::<(u8, u8, u8)>();

    for ((a, b, c), (d, e, f), (g, h, i)) in izip!(previous, window, next) {
        if e != b'@' {
            continue;
        }

        let adjacent = [a, b, c, d, e, f, g, h, i];
        let count = adjacent.iter().filter(|&&item| item == b'@').count() - 1;

        if count < 4 {
            answer += 1;
        }
    }

    Some(answer)
}

/// Builds a padded grid (with 1-cell border), represented by a contiguous boolean
/// vector, where `true` indicates a roll is present. This allows for efficient
/// unchecked neighbour access using pointer arithmetic and a fixed stride.
///
/// The initial scan is used to collect the starting positions of all rolls,
/// which is then used to efficiently jump between active rolls in subsequent iterations.
pub fn part_two(input: &str) -> Option<u64> {
    let mut roles = Vec::new();
    let mut positions = Vec::with_capacity(input.len());
    let mut new_positions = Vec::with_capacity(input.len());

    let line_length = input.find('\n')?;

    create_padded_grid(input, line_length, &mut roles, &mut positions);

    let stride = line_length as i16 + 2;
    let mut total_removed = 0;

    loop {
        let mut removed = 0;

        for &i in &positions {
            if is_removable(&roles, i, stride) {
                // SAFETY: i is always within bounds
                *unsafe { roles.get_unchecked_mut(i as usize) } = false;
                removed += 1;
            } else {
                new_positions.push(i);
            }
        }

        if removed == 0 {
            break;
        }

        total_removed += removed;

        // Swaps the two vectors, without reallocating
        mem::swap(&mut positions, &mut new_positions);
        new_positions.clear();
    }

    Some(total_removed)
}

/// Parses the input to create a 1-cell padded grid, represented as a boolean vector.
fn create_padded_grid(
    input: &str,
    line_length: usize,
    roles: &mut Vec<bool>,
    positions: &mut Vec<u16>,
) {
    // Add padding to the top of the grid
    roles.extend(std::iter::repeat_n(false, line_length + 2));

    // Pad the start of the first line
    roles.push(false);

    // Parse the input and track active roll positions
    for char in input.chars() {
        match char {
            '@' => {
                positions.push(roles.len() as u16);
                roles.push(true);
            }
            '.' => roles.push(false),

            // Pad end of line and start of next line
            '\n' => roles.extend([false, false]),

            _ => (),
        }
    }

    // If the input doesn't end with a newline, add it
    if !input.ends_with('\n') {
        roles.extend([false, false]);
    }

    // Add padding to the bottom of the grid
    roles.extend(std::iter::repeat_n(false, line_length + 2));
}

/// Returns true if the roll at index `i` has fewer than 4 adjacent rolls.
/// Skips bounds checking for performance; caller must ensure `i` is valid.
fn is_removable(roles: &[bool], i: u16, stride: i16) -> bool {
    let mut count = 0;
    let strides = [-stride, 0, stride];

    for s in strides {
        for o in [-1, 0, 1] {
            let idx = i.wrapping_add_signed(s).wrapping_add_signed(o);

            // SAFETY: all indices are within bounds due to padding
            count += unsafe { *roles.get_unchecked(idx as usize) } as u8;
        }
    }

    // the center roll is included, so < 5 means < 4 adjacent
    count < 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
