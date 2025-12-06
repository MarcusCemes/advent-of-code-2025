use itertools::izip;

advent_of_code::solution!(6);

const N_COLS: usize = 1000;
const N_ROWS: usize = 4;

pub fn part_one(input: &str) -> Option<u64> {
    let (mut data, ops) = parse_input_1(input);
    let answer = do_homework_1(&mut data, &ops);
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut lines, ops) = parse_input_2(input);
    let answer = do_homework_2(&mut lines, ops);
    Some(answer)
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

/* -- Part 2 -- */

fn parse_input_1(input: &str) -> (Vec<u64>, Vec<Operation>) {
    let mut data = Vec::with_capacity(N_ROWS * N_COLS);
    let mut ops = Vec::new();

    for line in input.lines() {
        let final_line = line.starts_with('*') || line.starts_with('+');

        if final_line {
            for op in line.split_ascii_whitespace() {
                let operation = match op {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    _ => continue,
                };

                ops.push(operation);
            }
        } else {
            for num in line.split_ascii_whitespace() {
                let value = num.parse().unwrap();

                data.push(value);
            }
        }
    }

    (data, ops)
}

fn do_homework_1(data: &mut [u64], ops: &[Operation]) -> u64 {
    let stride = ops.len();

    let (acc, rows) = data.split_at_mut(stride);

    for row in rows.chunks(stride) {
        for (lhs, rhs, op) in izip!(&mut *acc, row, ops) {
            *lhs = op.apply(*lhs, *rhs);
        }
    }

    data[..stride].iter().copied().sum()
}

/* -- Part 2 -- */

fn parse_input_2(input: &str) -> (Vec<&[u8]>, impl Iterator<Item = Operation>) {
    let mut lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let ops = lines.pop().unwrap().iter().flat_map(|b| match b {
        b'+' => Some(Operation::Add),
        b'*' => Some(Operation::Multiply),
        _ => None,
    });

    (lines, ops)
}

fn do_homework_2(lines: &mut [&[u8]], ops: impl Iterator<Item = Operation>) -> u64 {
    let mut answer = 0;
    let mut numbers = Vec::with_capacity(N_ROWS);

    for op in ops {
        loop {
            let mut number = 0;

            for line in &mut *lines {
                if let Some(digit) = line.split_off_first()
                    && (b'0'..=b'9').contains(&digit)
                {
                    number = 10 * number + (digit - b'0') as u64;
                }
            }

            if number == 0 {
                let result = (numbers.iter().copied())
                    .reduce(|acc, x| op.apply(acc, x))
                    .unwrap_or(0);

                answer += result;
                numbers.clear();
                break;
            }

            numbers.push(number);
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
