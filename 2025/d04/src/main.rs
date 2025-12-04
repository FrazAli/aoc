// 🧻 🚜 ۫·
// 󱅗 󰟉 

use std::collections::VecDeque;
use std::io::Result;

use common::{grid::DIRS_8, input::Input};

const NEIGHBOR_LIMIT: u8 = 4;

fn main() -> Result<()> {
    let input = Input::from_default()?;
    let grid = parse_grid(&input.raw);

    let part1 = accessible_rolls(&grid);
    println!("Part 1: {part1}");

    let part2 = removable_rolls(grid);
    println!("Part 2: {part2}");

    Ok(())
}

fn parse_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'@').collect())
        .collect()
}

fn neighbor_count(grid: &[Vec<bool>], r: usize, c: usize) -> u8 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut neighbors = 0u8;
    for &(dr, dc) in &DIRS_8 {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
            continue;
        }

        if grid[nr as usize][nc as usize] {
            neighbors += 1;
        }
    }

    neighbors
}

fn accessible_rolls(grid: &[Vec<bool>]) -> u32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }

    let mut count = 0u32;

    for r in 0..rows {
        for c in 0..grid[0].len() {
            if !grid[r][c] {
                continue;
            }

            let neighbors = neighbor_count(grid, r, c);
            if neighbors < NEIGHBOR_LIMIT {
                count += 1;
            }
        }
    }

    count
}

fn removable_rolls(mut grid: Vec<Vec<bool>>) -> u32 {
    let cols = grid[0].len();
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }

    // Pre-compute neighbor counts for each paper roll.
    let mut neighbor_counts = vec![vec![0u8; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] {
                neighbor_counts[r][c] = neighbor_count(&grid, r, c);
            }
        }
    }

    let mut queue = VecDeque::new();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] && neighbor_counts[r][c] < NEIGHBOR_LIMIT {
                queue.push_back((r, c));
            }
        }
    }

    let mut removed = 0u32;
    while let Some((r, c)) = queue.pop_front() {
        if !grid[r][c] {
            continue;
        }

        grid[r][c] = false;
        removed += 1;

        // update neighbor counts for adjacent rolls after current removal
        for &(dr, dc) in &DIRS_8 {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                continue;
            }

            let (nr_u, nc_u) = (nr as usize, nc as usize);
            if !grid[nr_u][nc_u] {
                continue;
            }

            if neighbor_counts[nr_u][nc_u] > 0 {
                neighbor_counts[nr_u][nc_u] -= 1;
            }

            if neighbor_counts[nr_u][nc_u] < NEIGHBOR_LIMIT {
                queue.push_back((nr_u, nc_u));
            }
        }
    }

    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_grid() -> Vec<Vec<bool>> {
        let example = concat!(
            "..@@.@@@@.\n",
            "@@@.@.@.@@\n",
            "@@@@@.@.@@\n",
            "@.@@@@..@.\n",
            "@@.@@@@.@@\n",
            ".@@@@@@@.@\n",
            ".@.@.@.@@@\n",
            "@.@@@.@@@@\n",
            ".@@@@@@@@.\n",
            "@.@.@@@.@.\n",
        );
        parse_grid(example)
    }

    #[test]
    fn example_accessible_count() {
        let grid = example_grid();
        assert_eq!(accessible_rolls(&grid), 13);
    }

    #[test]
    fn example_total_removed() {
        let grid = example_grid();
        assert_eq!(removable_rolls(grid), 43);
    }
}
