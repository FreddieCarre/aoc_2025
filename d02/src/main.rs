use rs_utils::input::read_input;

const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn get_num_digits(i: f64) -> f64 {
    i.log10().floor() + 1_f64
}

fn is_even_digits(i: i64) -> bool {
    let v: f64 = i as f64;
    let digits = get_num_digits(v);

    digits % 2_f64 == 0_f64
}

fn split(i: f64) -> (f64, f64) {
    let n = get_num_digits(i);
    let d = 10_f64.powf(n / 2_f64);

    let lhs = (i / d).floor();
    let rhs = i % d;

    (lhs, rhs)
}

fn get_factors(len: usize) -> Vec<usize> {
    let mid = len / 2;
    let mut results: Vec<usize> = vec![];

    for i in 1..mid + 1 {
        if len % i == 0 {
            results.push(i as usize);
        }
    }

    results
}

fn take_windows(window_len: usize, s: &String) -> Vec<String> {
    let mut results: Vec<String> = vec![];

    let mut i = 0;
    let mut j = 0 + window_len;

    let mut chars = s.chars();

    while j <= s.len() {
        let mut result: String = String::new();

        while i < j {
            let next: String = chars.next().unwrap().into();
            result += &next;

            i += 1;
        }

        j += window_len;

        results.push(result);
    }

    results
}

#[derive(Debug, Clone, Copy)]
struct Range(i64, i64);

impl Range {
    fn from_str(r: &str) -> Range {
        let parts: Vec<&str> = r.split("-").collect();

        if parts.len() != 2 {
            panic!("Invalid parts {:?}", parts);
        }

        let min: i64 = match parts[0].parse() {
            Ok(m) => m,
            Err(e) => panic!("Error parsing Range {}", e),
        };
        let max: i64 = match parts[1].parse() {
            Ok(m) => m,
            Err(e) => panic!("Error parsing Range {}", e),
        };

        Range(min, max)
    }

    fn has_even_length(self) -> bool {
        is_even_digits(self.0) || is_even_digits(self.1)
    }

    fn string_repetitions(self) -> i64 {
        let mut count = 0;

        let mut invalid: Vec<String> = vec![];

        for i in self.0..self.1 + 1 {
            let s = i.to_string();
            let factors = get_factors(s.len());

            let mut invalid_number = None;

            for window in factors.clone() {
                let w = take_windows(window, &s);

                let v = &w[0];

                if w.iter().all(|x| x == v) {
                    invalid_number = Some(s);

                    break;
                }
            }

            if let Some(number) = invalid_number {
                invalid.push(number);
                count += i;
            }
        }

        count
    }

    fn symmetry(self) -> i64 {
        let mut count = 0;

        for i in self.0..self.1 {
            if !is_even_digits(i) {
                continue;
            }

            let s = split(i as f64);

            if s.0 == s.1 {
                count += i;
            }
        }

        count
    }
}

#[derive(Debug)]
struct Puzzle {
    entries: Vec<Range>,
}

impl Puzzle {
    fn from_str(i: &str) -> Puzzle {
        let entries = i.split(",").map(Range::from_str).collect();

        Puzzle { entries }
    }

    fn solve(self) -> i64 {
        self.entries
            .iter()
            .filter_map(|f| {
                if !f.has_even_length() {
                    None
                } else {
                    Some(f.symmetry())
                }
            })
            .sum()
    }

    fn solve_2(self) -> i64 {
        self.entries
            .iter()
            .filter_map(|f| Some(f.string_repetitions()))
            .sum()
    }
}

fn main() {
    let i = read_input("./input.txt");
    let input_1 = Puzzle::from_str(&i);
    let input_test = Puzzle::from_str(TEST_INPUT);

    println!("part 1 test -> {:?}", input_test.solve());
    println!("part 1 test -> {:?}", input_1.solve());

    let input_2 = Puzzle::from_str(&i);
    let input_test_2 = Puzzle::from_str(TEST_INPUT);

    println!("part 2 test -> {:?}", input_test_2.solve_2());
    println!("part 2 test -> {:?}", input_2.solve_2());
}
