use std::fmt;
use std::fmt::{Display, Formatter};
use crate::days::day06::Direction::{Down, Left, Right, Up};
use crate::days::day06::State::{Occupied, Unvisited, Visited};
use crate::etc::read_input;
use crate::{Solution, SolutionPair};
use log::trace;

pub fn solve() -> SolutionPair {
    let input = read_input("day_06_1");
    let grid = Grid::parse(input);
    let sol1: u64 = sol1(&mut grid.clone());
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn sol1(grid: &mut Grid) -> u64 {
    grid.move_guard();
    count_visited(&grid)
}

fn count_visited(grid: &Grid) -> u64 {
    grid.grid
        .iter()
        .map(|line| line.iter().filter(|space| **space == Visited).count() as u64)
        .sum()
}

struct Grid {
    guard: Guard,
    size: (usize, usize),
    grid: Vec<Vec<State>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        let len = &self.size.0 as u64;
        for i in &0..&self.grid.len() {
            write!(f, "{} ", i)?;
        }
        for y in 0..self.grid.len() {
            write!(f, "{}", y as u64)?;
            for x in 0..self.grid[y].len() {
                if self.guard.x as usize == x && self.guard.y as usize == y {
                    let symbol = match self.guard.direction {
                        Up => '^',
                        Right => '>',
                        Down => 'v',
                        Left => '<',
                    };
                    write!(f, "{}", symbol)?;
                } else {
                    let symbol = match self.grid[y][x] {
                        Unvisited => '.',
                        Occupied => '#',
                        Visited => 'X',
                    };
                    write!(f, "{}", symbol)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            guard: self.guard.clone(),
            size: self.size,
            grid: self.grid.clone(),
        }
    }
}

impl Grid {
    fn parse(input: String) -> Grid {
        let mut guard = None;
        let mut y_vec = Vec::new();
        let char_grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        for y in 0..char_grid.len() {
            let mut x_vec = Vec::new();
            for x in 0..char_grid[y].len() {
                match char_grid[y][x] {
                    '.' => x_vec.push(Unvisited),
                    '#' => x_vec.push(Occupied),
                    'X' => x_vec.push(Visited),
                    other => {
                        let direction = Direction::parse(other);
                        guard = Some(Guard::new(x as u64, y as u64, direction));
                    }
                }
            }
            y_vec.push(x_vec);
        }
        Grid {
            guard: guard.unwrap(),
            size: (y_vec.len(), y_vec[0].len()),
            grid: y_vec,
        }
    }

    fn move_guard(&mut self) {
        loop {
            trace!("{}", self);
            let (x,y) = self.guard.direction.next_coordinate((self.guard.x, self.guard.y));
            self.guard.x = x;
            self.guard.y = y;
            if !self.is_within_boundaries(&self.guard) {
                break;
            }
            self.visit((self.guard.x, self.guard.y));
            let next_coordinate = self
                .guard
                .direction
                .next_coordinate((self.guard.x, self.guard.y));
            if next_coordinate.0 as usize >= self.size.0
                || next_coordinate.1 as usize >= self.size.1
            {
                break;
            }
            match self.grid[next_coordinate.0 as usize][next_coordinate.1 as usize] {
                Occupied => {
                    self.guard.direction = self.guard.direction.next();
                }
                _ => {}
            }
        }
    }

    fn visit(&mut self, coordinate: (u64, u64)) {
        if coordinate.0 as usize >= self.size.0 || coordinate.1 as usize >= self.size.1 {
            return;
        }
        match self.grid[coordinate.0 as usize][coordinate.1 as usize] {
            Occupied => {}
            _ => {
                self.grid[coordinate.0 as usize][coordinate.1 as usize] = Visited;
            }
        }
    }

    fn is_within_boundaries(&self, guard: &Guard) -> bool {
        let within_boundaries = guard.x < self.size.0 as u64 && guard.y < self.size.1 as u64;
        trace!(
            "Guard: {:?}, Grid-size: {:?} -> Within boundaries: {:?}",
            guard,
            self.size,
            within_boundaries
        );
        within_boundaries
    }
}

#[derive(Clone, Debug)]
struct Guard {
    x: u64,
    y: u64,
    direction: Direction,
}

impl Guard {
    fn new(x: u64, y: u64, direction: Direction) -> Self {
        Self { x, y, direction }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    Unvisited,
    Visited,
    Occupied,
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Up => {"^"}
            Right => {">"}
            Down => {"v"}
            Left => {"<"}
        };
        Ok(write!(f, "{}", ch)?)
    }
}

impl Direction {
    fn parse(ch: char) -> Direction {
        match ch {
            '^' => Up,
            '>' => Right,
            'v' => Down,
            '<' => Left,
            _ => Up,
        }
    }
    fn next(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn next_coordinate(&self, (x, y): (u64, u64)) -> (u64, u64) {
        match self {
            Up => (x, y.wrapping_sub(1)),
            Right => (x.wrapping_add(1), y),
            Down => (x, y.wrapping_add(1)),
            Left => (x.wrapping_sub(1), y),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day06::{sol1, Grid};
    use crate::etc::solution::initialize_logging;

    #[test]
    fn test_input() {
        initialize_logging();
        let test_input = "....#.....
             .........#
             ..........
             ..#.......
             .......#..
             ..........
             .#..^.....
             ........#.
             #.........
             ......#...";
        let trimmed_input = test_input.replace(" ", "");
        let mut test_grid = Grid::parse(trimmed_input);
        assert_eq!(sol1(&mut test_grid), 41);
    }
}
