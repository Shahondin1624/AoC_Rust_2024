use crate::etc::read_input;
use crate::{Solution, SolutionPair};
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_input("day_04_1");
    let sol1: u64 = sol1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn sol1(input: &String) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    count_xmas(&grid) as u64
}

fn sol2(input: &String) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    count_x_mas(&grid) as u64
}

fn count_xmas(grid: &[Vec<char>]) -> usize {
    let directions = [
        (1, 0),   // right
        (0, 1),   // down
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 0),  // left
        (0, -1),  // up
        (-1, -1), // up-left
        (-1, 1),  // up-right
    ];
    let word = "XMAS".chars().collect::<Vec<_>>();
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for &(dx, dy) in &directions {
                if check_word(grid, &word, i as isize, j as isize, dx, dy) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_word(grid: &[Vec<char>], word: &[char], x: isize, y: isize, dx: isize, dy: isize) -> bool {
    for (i, &ch) in word.iter().enumerate() {
        let nx = x + i as isize * dx;
        let ny = y + i as isize * dy;
        if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid[0].len() as isize {
            return false;
        }
        if grid[nx as usize][ny as usize] != ch {
            return false;
        }
    }
    true
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if check_x_mas(grid, i as isize, j as isize) {
                count += 1;
            }
        }
    }
    count
}
fn check_x_mas(grid: &[Vec<char>], x: isize, y: isize) -> bool {
    let patterns = [
        [(0, -1), (1, 0), (0, 1)],  // MAS pattern
        [(0, 1), (1, 0), (0, -1)],  // SAM pattern
        [(0, -1), (-1, 0), (0, 1)], // MAS pattern (reversed)
        [(0, 1), (-1, 0), (0, -1)], // SAM pattern (reversed)
    ];
    for pattern in &patterns {
        if grid[x as usize][y as usize] == 'A'
            && grid[(x + pattern[0].0) as usize][(y + pattern[0].1) as usize] == 'M'
            && grid[(x + pattern[1].0) as usize][(y + pattern[1].1) as usize] == 'S'
            && grid[(x + pattern[2].0) as usize][(y + pattern[2].1) as usize] == 'M'
        {
            return true;
        }
    }
    false
}
