use rs_utils::input::read_input;

const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

fn parse(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

/// Find the next maximum digit within the valid search window.
/// Returns the digit value and the index where it was found.
fn get_next_max_digit(batteries: &[u8], start_index: usize, remaining_count: usize) -> (u8, usize) {
    // Calculate the valid search window:
    // We must leave enough positions for the remaining picks after this one
    let end_index = batteries.len() - (remaining_count - 1);
    let search_window = &batteries[start_index..end_index];

    // Find the maximum digit and its position within the window
    // Use fold to get the FIRST maximum (max_by_key returns the last)
    let (local_idx, max_digit) = search_window.iter().enumerate().fold(
        (0, search_window[0]),
        |(best_idx, best_val), (idx, &val)| {
            if val > best_val {
                (idx, val)
            } else {
                (best_idx, best_val)
            }
        },
    );

    (max_digit, start_index + local_idx)
}

/// Calculate the maximum joltage by greedily selecting `num` digits.
fn get_max_joltage(batteries: &[u8], num: usize) -> u64 {
    let mut result: u64 = 0;
    let mut last_index = 0;

    for remaining in (1..=num).rev() {
        let (digit, found_index) = get_next_max_digit(batteries, last_index, remaining);
        result = result * 10 + digit as u64;
        last_index = found_index + 1;
    }

    result
}

struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn new(input: &str) -> Self {
        let batteries = input
            .chars()
            .map(|c| c.to_digit(10).expect("expected digit") as u8)
            .collect();

        Bank { batteries }
    }

    fn max_joltage(&self, num: usize) -> u64 {
        get_max_joltage(&self.batteries, num)
    }
}

impl std::fmt::Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.batteries.iter().map(|&b| (b + b'0') as char).collect();
        write!(f, "Bank {{ {} }}", s)
    }
}

fn part_1(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|line| Bank::new(line).max_joltage(2))
        .sum()
}

fn part_2(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|line| Bank::new(line).max_joltage(12))
        .sum()
}

fn main() {
    let input = read_input("./input.txt");
    println!("Part 1 test: {}", part_1(TEST_INPUT));
    println!("Part 1: {}", part_1(&input));
    println!("Part 2 test: {}", part_2(TEST_INPUT));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 357);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 3121910778619);
    }

    #[test]
    fn test_get_max_joltage_2_cell() {
        let bank = Bank::new("987654321111111");
        assert_eq!(bank.max_joltage(2), 98);
    }

    #[test]
    fn test_get_max_joltage_12_cell_one() {
        let bank = Bank::new("987654321111111");
        assert_eq!(bank.max_joltage(12), 987654321111);
    }

    #[test]
    fn test_get_max_joltage_12_cell_two() {
        let bank = Bank::new("811111111111119");
        assert_eq!(bank.max_joltage(12), 811111111119);
    }

    #[test]
    fn test_get_max_joltage_12_cell_three() {
        let bank = Bank::new("234234234234278");
        assert_eq!(bank.max_joltage(12), 434234234278);
    }

    #[test]
    fn test_get_max_joltage_12_cell_four() {
        let bank = Bank::new("818181911112111");
        assert_eq!(bank.max_joltage(12), 888911112111);
    }
}
