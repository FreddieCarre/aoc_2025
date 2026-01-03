use std::fmt;

use rs_utils::input::read_input;

const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

#[derive(Clone, Copy, Debug, PartialEq)]
struct FreshRange {
    min: u64,
    max: u64,
}

impl fmt::Display for FreshRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Min {} - Max {}", self.min, self.max)
    }
}

impl FreshRange {
    fn from_str(s: &str) -> Self {
        let parts: Vec<u64> = s.split("-").map(|p| p.parse().unwrap()).collect();

        if parts.len() != 2 {
            panic!("FreshRange string is in wrong format {}", s);
        }

        let min = *parts.iter().min().unwrap();
        let max = *parts.iter().max().unwrap();

        FreshRange { min, max }
    }

    fn contains(&self, id: u64) -> bool {
        self.min <= id && self.max >= id
    }

    fn combine(&self, other: &FreshRange) -> Result<FreshRange, ()> {
        // No overlap at all
        if self.min > other.max || self.max < other.min {
            Err(())
        } else {
            let new_min = self.min.min(other.min);
            let new_max = self.max.max(other.max);

            Ok(FreshRange {
                min: new_min,
                max: new_max,
            })
        }
    }
}

fn parse(input: &str) -> (Vec<FreshRange>, Vec<u64>) {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    if parts.len() != 2 {
        panic!("Input is not in correct format {}", input);
    }

    let fresh_ranges: Vec<FreshRange> = parts[0].split("\n").map(FreshRange::from_str).collect();
    let ids: Vec<u64> = parts[1].split("\n").map(|id| id.parse().unwrap()).collect();

    (fresh_ranges, ids)
}

fn part_1(input: &str) -> usize {
    let mut count = 0;
    let (fresh_ranges, ids) = parse(input);

    for id in ids {
        let is_fresh = fresh_ranges.iter().any(|fr| fr.contains(id));
        if is_fresh {
            count += 1;
        }
    }

    count
}

fn part_2(input: &str) -> u64 {
    let (mut fresh_ranges, _) = parse(input);

    // Sort ranges by min BEFORE merging so overlapping ranges become adjacent
    fresh_ranges.sort_by(|a, b| a.min.cmp(&b.min));

    let mut continue_to_merge = true;

    while continue_to_merge {
        let mut has_merged = false;
        let mut i = 0;

        while i < fresh_ranges.len() - 1 {
            let j = i + 1;

            let slf = fresh_ranges[i];
            let other = fresh_ranges[j];

            let new = slf.combine(&other);

            if let Ok(new_range) = new {
                has_merged = true;
                fresh_ranges[i] = new_range;
                fresh_ranges.remove(j);
            }

            i += 1;
        }

        if !has_merged {
            continue_to_merge = false;
        }
    }

    fresh_ranges.iter().map(|r| r.max - r.min + 1).sum()
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
        assert_eq!(part_1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 14);
    }

    #[test]
    fn test_combine_no_overlap() {
        let fresh_range_one = FreshRange { min: 3, max: 5 };
        let fresh_range_two = FreshRange { min: 6, max: 11 };

        assert_eq!(fresh_range_one.combine(&fresh_range_two).err(), Some(()));
    }

    #[test]
    fn test_combine_complete_overlap() {
        let fresh_range_one = FreshRange { min: 3, max: 12 };
        let fresh_range_two = FreshRange { min: 6, max: 11 };

        assert_eq!(
            fresh_range_one.combine(&fresh_range_two),
            Ok(fresh_range_one)
        );
    }

    #[test]
    fn test_combine_partial_overlap() {
        let fresh_range_one = FreshRange { min: 3, max: 12 };
        let fresh_range_two = FreshRange { min: 1, max: 10 };

        assert_eq!(
            fresh_range_one.combine(&fresh_range_two),
            Ok(FreshRange { min: 1, max: 12 })
        );
    }
}
