use pathfinding::prelude::*;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let mut answer = 0;

    for machine in parse_input(input) {
        let path = bfs(
            &0,
            |&p| machine.buttons.iter().map(move |&b| p ^ b),
            |&p| p == machine.pattern,
        );

        answer += path.unwrap().len() as u64 - 1;
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Debug)]
struct Machine {
    buttons: Box<[u16]>,
    joltage: Box<[u16]>,
    pattern: u16,
}

fn parse_input(input: &str) -> impl Iterator<Item = Machine> {
    input.lines().map(|line| {
        let mut buttons = Vec::new();

        let mut parts = line.split_ascii_whitespace();
        let pattern_str = parts.next().unwrap();

        let pattern_len = (pattern_str.len() - 2) as u16;
        let pattern = pattern_str[1..pattern_str.len() - 1]
            .chars()
            .fold(0, |acc, s| acc << 1 | (s == '#') as u16);

        for part in parts {
            if part.starts_with('(') {
                let sequence = part[1..part.len() - 1].split(',').fold(0, |acc, s| {
                    let i = s.parse::<u16>().unwrap();
                    acc | 1 << (pattern_len - 1 - i)
                });

                buttons.push(sequence);
            } else {
                let joltage = part[1..part.len() - 1]
                    .split(',')
                    .map(|s| s.parse::<u16>().unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice();

                return Machine {
                    buttons: buttons.into_boxed_slice(),
                    joltage,
                    pattern,
                };
            }
        }

        unreachable!()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
