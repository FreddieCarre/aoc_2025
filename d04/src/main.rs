use rs_utils::input::read_input;

const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

struct Point {
    x: usize,
    y: usize,
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Grid {
    raw: Vec<Vec<char>>,

    width: usize,
    height: usize,

    /// Positions of all '@' characters in the grid. Coordinates are (x, y)
    positions: Vec<Point>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.raw {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn from_raw(raw: Vec<Vec<char>>) -> Self {
        let width = raw[0].len();
        let height = raw.len();
        let positions = raw
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == '@')
                    .map(move |(x, _)| Point { x, y })
            })
            .collect();

        Self {
            raw,
            width,
            height,
            positions,
        }
    }

    fn visit(&mut self, x: usize, y: usize) {
        self.raw[y][x] = 'x';
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<Point> {
        let mut neighbours = Vec::new();

        // top left
        if x > 0 && y > 0 {
            let p = Point { x: x - 1, y: y - 1 };

            if self.positions.contains(&p) {
                neighbours.push(p);
            }
        }

        // top
        if y > 0 {
            let p = Point { x: x, y: y - 1 };

            if self.positions.contains(&p) {
                neighbours.push(p);
            }
        }

        // top right
        if x < self.width - 1 && y > 0 {
            let p = Point { x: x + 1, y: y - 1 };

            if self.positions.contains(&p) {
                neighbours.push(p);
            }
        }

        // right
        if x < self.width - 1 {
            let p = Point { x: x + 1, y };

            if self.positions.contains(&p) {
                neighbours.push(p);
            }
        }

        // bottom right
        if x < self.width - 1 && y < self.height - 1 {
            let p = Point { x: x + 1, y: y + 1 };

            if self.positions.contains(&p) {
                neighbours.push(p);
            }
        }

        // bottom
        if y < self.height - 1 {
            let p = Point { x: x, y: y + 1 };

            if self.positions.contains(&p) {
                neighbours.push(p);
            }
        }

        // bottom left
        if x > 0 && y < self.height - 1 {
            let p = Point { x: x - 1, y: y + 1 };

            if self.positions.contains(&p) {
                neighbours.push(p);
            }
        }

        // left
        if x > 0 {
            let p = Point { x: x - 1, y };

            if self.positions.contains(&p) {
                neighbours.push(p);
            }
        }

        neighbours
    }
}

fn part_1(input: &str, is_test: bool) -> usize {
    let mut grid = Grid::from_raw(parse(input));
    let mut count = 0;

    if is_test {
        println!("Grid start:\n{}", grid);
    }

    for position in grid.positions.clone() {
        let neighbours = grid.neighbours(position.x, position.y);

        if neighbours.len() < 4 {
            grid.visit(position.x, position.y);
            count += 1;
        }
    }

    if is_test {
        println!("Grid finish:\n{}", grid);
    }

    count
}

fn part_2(input: &str, is_test: bool) -> usize {
    let mut grid = Grid::from_raw(parse(input));
    let mut count = 0;
    let mut removed = true;

    if is_test {
        println!("Grid start:\n{}", grid);
    }

    while removed {
        let mut iter_count = 0;

        for position in grid.positions.clone() {
            let neighbours = grid.neighbours(position.x, position.y);

            if neighbours.len() < 4 {
                grid.visit(position.x, position.y);
                iter_count += 1;
            }
        }

        grid = Grid::from_raw(grid.raw);

        count += iter_count;

        if iter_count == 0 {
            removed = false;
        }
    }

    if is_test {
        println!("Grid finish:\n{}", grid);
    }

    count
}

fn main() {
    let input = read_input("./input.txt");
    println!("Part 1 test: {}", part_1(TEST_INPUT, true));
    println!("Part 1: {}", part_1(&input, false));

    println!("Part 2 test: {}", part_2(TEST_INPUT, true));
    println!("Part 2: {}", part_2(&input, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT, true), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT, true), 43);
    }
}
