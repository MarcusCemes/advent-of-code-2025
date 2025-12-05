advent_of_code::solution!(1);

const POSITIONS: i64 = 100;
const START_POSITION: i64 = 50;

pub fn part_one(input: &str) -> Option<u64> {
    let mut clicks = 0;
    let mut state = START_POSITION;

    for steps in input.lines().map(parse_line) {
        state = advance(state, steps);

        if state == 0 {
            clicks += 1;
        }
    }

    Some(clicks)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut clicks = 0;
    let mut state = START_POSITION;

    for steps in input.lines().map(parse_line) {
        clicks += zero_hits(state, steps);
        state = advance(state, steps);
    }

    Some(clicks)
}

fn advance(state: i64, delta: i64) -> i64 {
    (state + delta).rem_euclid(POSITIONS)
}

/// Counts how many zeros we hit while moving `steps` slots along the circular track.
/// Measures the distance to the next clickable zero, then one for each full lap.
fn zero_hits(state: i64, steps: i64) -> u64 {
    let dir = steps.signum();
    let count = steps.abs();

    // Calculate distance to the next zero in the given direction
    let mut dist_next_zero = if dir > 0 {
        (POSITIONS - state) % POSITIONS
    } else {
        state % POSITIONS
    };

    // If we started on zero, we consider the first hit to be one full lap away
    if dist_next_zero == 0 {
        dist_next_zero = POSITIONS;
    }

    // If we don't have enough steps to reach the first zero, no clicks
    if dist_next_zero > count {
        return 0;
    }

    // Count one for the first hit, then one for each full lap after that
    (1 + (count - dist_next_zero) / POSITIONS) as u64
}

fn parse_line(line: &str) -> i64 {
    let left = line.starts_with('L');
    let mut count = line[1..].parse().unwrap_or(0);

    if left {
        count = -count;
    }

    count
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
        assert_eq!(result, Some(6));
    }
}
