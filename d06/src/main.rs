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

/// A problem parsed as a grid of characters, preserving column positions.
/// Each inner Vec represents a row, and each char is a column position.
struct Problem {
    /// The character grid (rows x cols), NOT including the operator row
    grid: Vec<Vec<char>>,
    /// The operation for this problem
    op: Operation,
}

/// Parse input into a character grid, then group columns into problems.
/// Problems are separated by columns that are entirely spaces.
fn parse_problems(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        panic!("Empty input");
    }

    // Find the maximum line length to handle ragged lines
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Convert to a character grid, padding shorter lines with spaces
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let row_count = grid.len();
    let col_count = max_len;

    if row_count < 2 {
        panic!("Need at least 2 rows (numbers + operators)");
    }

    // The last row contains operators
    let op_row = &grid[row_count - 1];
    let data_rows = &grid[0..row_count - 1];

    // Group columns into problems by finding separator columns (all spaces)
    let mut problems: Vec<Problem> = Vec::new();
    let mut current_cols: Vec<usize> = Vec::new();

    for col in 0..col_count {
        // Check if this column is a separator (all spaces including operator row)
        let is_separator = data_rows.iter().all(|row| row[col] == ' ') && op_row[col] == ' ';

        if is_separator {
            // If we have accumulated columns, create a problem
            if !current_cols.is_empty() {
                let problem = create_problem(data_rows, op_row, &current_cols);
                problems.push(problem);
                current_cols.clear();
            }
        } else {
            current_cols.push(col);
        }
    }

    // Don't forget the last problem if there's no trailing separator
    if !current_cols.is_empty() {
        let problem = create_problem(data_rows, op_row, &current_cols);
        problems.push(problem);
    }

    problems
}

fn create_problem(data_rows: &[Vec<char>], op_row: &[char], cols: &[usize]) -> Problem {
    // Extract the grid for this problem
    let grid: Vec<Vec<char>> = data_rows
        .iter()
        .map(|row| cols.iter().map(|&c| row[c]).collect())
        .collect();

    // Find the operator (first non-space char in the operator row for these columns)
    let op_char = cols
        .iter()
        .find_map(|&c| {
            let ch = op_row[c];
            if ch != ' ' { Some(ch) } else { None }
        })
        .expect("No operator found for problem");

    let op = match op_char {
        '*' => Operation::MUL,
        '+' => Operation::SUM,
        o => panic!("Unexpected operation {o}"),
    };

    Problem { grid, op }
}

/// Part 1: Read numbers left-to-right within each row
fn problem_to_equation_part1(problem: &Problem) -> Equation {
    let nums: Vec<u64> = problem
        .grid
        .iter()
        .map(|row| {
            let num_str: String = row.iter().filter(|c| c.is_ascii_digit()).collect();
            num_str.parse().expect("Failed to parse number")
        })
        .collect();

    Equation {
        nums,
        op: match problem.op {
            Operation::SUM => Operation::SUM,
            Operation::MUL => Operation::MUL,
        },
    }
}

/// Part 2: Read columns right-to-left, building numbers from top to bottom within each column
fn problem_to_equation_part2(problem: &Problem) -> Equation {
    let col_count = problem.grid.first().map(|r| r.len()).unwrap_or(0);
    let row_count = problem.grid.len();

    // Process columns from right to left
    let nums: Vec<u64> = (0..col_count)
        .rev()
        .map(|col| {
            // Build number from digits in this column
            // Top digit is most significant, bottom is least significant
            // First collect all digits, then build the number
            let digits: Vec<u64> = (0..row_count)
                .filter_map(|row| {
                    let ch = problem.grid[row][col];
                    if ch.is_ascii_digit() {
                        Some(ch.to_digit(10).unwrap() as u64)
                    } else {
                        None
                    }
                })
                .collect();

            // Build number: first digit is most significant
            let mut num: u64 = 0;
            for digit in digits {
                num = num * 10 + digit;
            }

            num
        })
        .collect();

    Equation {
        nums,
        op: match problem.op {
            Operation::SUM => Operation::SUM,
            Operation::MUL => Operation::MUL,
        },
    }
}

fn part_1(input: &str) -> u64 {
    let problems = parse_problems(input);

    problems
        .iter()
        .map(|p| problem_to_equation_part1(p))
        .map(|e| e.solve())
        .sum()
}

fn part_2(input: &str) -> u64 {
    let problems = parse_problems(input);

    problems
        .iter()
        .map(|p| problem_to_equation_part2(p))
        .map(|e| e.solve())
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
