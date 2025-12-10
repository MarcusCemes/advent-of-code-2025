use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, constraint, default_solver, variable,
};
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
    let mut answer = 0;

    for machine in parse_input(input) {
        let n_buttons = machine.buttons.len();
        let n_joltages = machine.joltage.len();

        // Create a variable x[i] per button (number of presses)
        let mut vars = ProblemVariables::new();

        let x = (0..n_buttons)
            .map(|_| vars.add(variable().integer().min(0)))
            .collect::<Vec<_>>();

        // Create the minimisation objective (number of button presses: sum of x[i])
        let objective: Expression = x.iter().copied().sum();
        let mut problem = vars.minimise(objective).using(default_solver);

        // Our problem has a constraint Ax=b, this translates to the system
        // of equations: sum_i A[j][i] * x[i] == b[j], where j in [0, n_joltages)

        for i in 0..n_joltages {
            let mut expr = Expression::default();

            for (j, &btn_mask) in machine.buttons.iter().enumerate() {
                if btn_mask & (1 << (n_joltages - 1 - i)) != 0 {
                    expr += x[j];
                }
            }

            let constraint = constraint!(expr == machine.joltage[i] as i32);
            problem.add_constraint(constraint);
        }

        let solution = problem.solve().unwrap();
        let total = x.iter().map(|&v| solution.value(v)).sum::<f64>();

        answer += total.round() as u64;
    }

    Some(answer)
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
        assert_eq!(result, Some(33));
    }
}
