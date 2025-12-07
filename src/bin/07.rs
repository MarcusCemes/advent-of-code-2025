advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let (splits, _) = simulate_tachyon_manifold(input)?;
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, timelines) = simulate_tachyon_manifold(input)?;
    Some(timelines)
}

fn simulate_tachyon_manifold(input: &str) -> Option<(u64, u64)> {
    let mut input = input.bytes();

    let start_idx = input.position(|x| x == b'S')?;
    let line_length = 1 + start_idx + input.position(|x| x == b'\n')?;

    let mut beams = vec![0; line_length];
    let mut cursor = 0;
    let mut splits = 0;

    // SAFETY: start_idx is guaranteed to be within bounds
    unsafe { *beams.get_unchecked_mut(start_idx) = 1 };

    for char in input {
        if char == b'^' {
            // SAFETY: each line has the same length, this will never go OOB
            let timelines = unsafe { *beams.get_unchecked(cursor) };

            if timelines > 0 {
                splits += 1;

                // SAFETY: the input is padded with walls, so we will never go OOB
                unsafe {
                    *beams.get_unchecked_mut(cursor - 1) += timelines;
                    *beams.get_unchecked_mut(cursor) = 0;
                    *beams.get_unchecked_mut(cursor + 1) += timelines;
                }
            }
        }

        cursor += 1;

        if char == b'\n' {
            cursor = 0;
        }
    }

    let timelines = beams.iter().sum();

    Some((splits, timelines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
