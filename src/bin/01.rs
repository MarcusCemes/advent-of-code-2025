advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut clicks = 0;
    let mut state = 50;

    for steps in input.lines().map(parse_line) {
        state = (state + steps + 100) % 100;

        if state == 0 {
            clicks += 1;
        }
    }

    Some(clicks)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut clicks = 0;
    let mut state = 50;

    // Simulate each individual step. It's much simpler than
    // ranges and edge cases, and fast enough.
    for steps in input.lines().map(parse_line) {
        let count = steps.abs();
        let dir = steps.signum();

        for _ in 0..count {
            state = (state + dir + 100) % 100;

            if state == 0 {
                clicks += 1;
            }
        }
    }

    Some(clicks)
}

fn parse_line(line: &str) -> i32 {
    let left = line.starts_with('L');
    let mut count = line[1..].trim().parse().unwrap_or(0);

    if left {
        count *= -1;
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
