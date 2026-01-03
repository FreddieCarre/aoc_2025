use std::fmt::{self, Debug, Display};

use rs_utils::input::read_input;

const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

enum Operation {
    SUM,
    MUL,
}

impl Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SUM => write!(f, "+"),
            Self::MUL => write!(f, "*"),
        }
    }
}

struct Equation {
    nums: Vec<u64>,
    op: Operation,
}

impl Equation {
    fn solve(&self) -> u64 {
        match self.op {
            Operation::MUL => self.nums.iter().product(),
            Operation::SUM => self.nums.iter().sum(),
        }
    }
}

impl Debug for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .nums
            .iter()
            .map(|n| format!("{n}"))
            .collect::<Vec<String>>()
            .join(format!(" {} ", self.op).as_str());

        write!(f, "{s}")
    }
}

/// TODO: For part 2 - need to split only on the 1st whitespace between the groups of numbers.
/// i.e. if there is "123  4" then we should have "123" and " 4" - this is so that we get the correct position of the s.f.
/// part 2 will alos need to keep the items as strung so can't return as an equation yet.
fn parse(input: &str) -> Vec<Equation> {
    let lines: Vec<Vec<String>> = input
        .trim()
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    if lines.len() == 0 {
        panic!("Empty input {}", input);
    }

    let first = lines.first().unwrap();

    let row_count = lines.len();
    let col_count = first.len();

    (0..col_count)
        .map(|col| {
            let nums = lines[0..row_count - 1]
                .iter()
                .map(|l| l[col].parse().unwrap())
                .collect::<Vec<u64>>();

            let op = match lines.last().unwrap()[col].as_str() {
                "*" => Operation::MUL,
                "+" => Operation::SUM,
                o => panic!("Unexpected operation {o}"),
            };

            Equation { nums, op }
        })
        .collect()
}

fn part_1(input: &str) -> u64 {
    let equations = parse(input);

    equations.iter().map(|e| e.solve()).sum()
}

fn part_2(input: &str) -> u64 {
    let equations = parse(input);

    equations
        .iter()
        .map(|e| {
            println!("Initial equation {e:?}");
            let max_sf = e.nums.iter().max_by_key(|n| n.ilog10()).unwrap().ilog10();

            if max_sf == 0 {
                return e.solve();
            }

            let new_nums_str: Vec<Vec<String>> = e
                .nums
                .iter()
                .map(|n| format!("{n}").chars().map(|c| c.to_string()).collect())
                .collect();

            println!("Max sf {max_sf}");

            let mut new_nums: Vec<u64> = vec![];

            for sf in (0..=max_sf).rev() {
                let mut new_num: u64 = 0;
                let mut new_sf: u32 = 0;

                new_nums_str.iter().for_each(|parts| {
                    let next = &parts[sf as usize];

                    if next == "_" {
                        return;
                    }

                    let n_parsed: u64 = parts[sf as usize].parse().unwrap();

                    new_num += n_parsed * (10_u64).pow(new_sf);
                    new_sf += 1;
                });

                new_nums.push(new_num);
            }

            let new_e = Equation {
                nums: new_nums,
                op: match e.op {
                    Operation::SUM => Operation::SUM,
                    Operation::MUL => Operation::MUL,
                },
            };

            println!("New equation {new_e:?}");

            new_e.solve()
        })
        .sum()
}

fn main() {
    let input = read_input("./input.txt");

    println!("Part 1 test {}", part_1(TEST_INPUT));
    println!("Part 1 {}", part_1(&input));

    println!("Part 2 test {}", part_2(TEST_INPUT));
    println!("Part 2 {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 4277556);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 3263827);
    }
}
