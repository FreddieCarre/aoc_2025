use std::{char, fmt::Display};

use rs_utils::input::read_input;

const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

struct Position {
    x: usize,
    y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

struct Grid {
    height: usize,
    width: usize,

    beams: Vec<Position>,

    grid: Vec<Vec<char>>,

    splits: u64,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let height = grid.len();
        let width = grid[0].len();
        let start_x = grid[0].iter().position(|&x| x == 'S').unwrap();

        Grid {
            beams: vec![Position { x: start_x, y: 0 }],
            grid,
            height,
            splits: 0,
            width,
        }
    }

    fn next(&mut self) -> Option<()> {
        let mut new_beams: Vec<Position> = vec![];

        self.beams.iter().for_each(|beam| {
            let next_y = beam.y + 1;

            if next_y == self.height {
                return;
            }

            let next = self.grid[next_y][beam.x];

            match next {
                '.' => {
                    self.grid[next_y][beam.x] = '|';
                    new_beams.push(Position {
                        x: beam.x,
                        y: next_y,
                    });
                }
                '|' => {}
                '^' => {
                    let left = beam.x - 1;
                    let right = beam.x + 1;

                    self.splits += 1;
                    if left > 0 && self.grid[next_y][left] == '.' {
                        new_beams.push(Position { x: left, y: next_y });
                        self.grid[next_y][left] = '|';
                    }

                    if right < self.width && self.grid[next_y][right] == '.' {
                        new_beams.push(Position {
                            x: right,
                            y: next_y,
                        });
                        self.grid[next_y][right] = '|';
                    }
                }
                _ => {
                    panic!("haven't handled char {}", next);
                }
            }
        });

        if new_beams.len() > 0 {
            self.beams = new_beams;
            Some(())
        } else {
            None
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.grid {
            writeln!(f, "{}", l.iter().collect::<String>())?;
        }

        Ok(())
    }
}

fn part_1(input: &str) -> u64 {
    let mut grid = Grid::parse(input);

    let mut has_next = grid.next();

    while has_next.is_some() {
        has_next = grid.next();
    }

    grid.splits
}

fn part_2(input: &str) -> u64 {
    0
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
        assert_eq!(part_1(TEST_INPUT), 21);
    }

    #[test]
    #[ignore = "not implemented"]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 3263827);
    }
}
