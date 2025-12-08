use common::input::Input;

fn find_start(lines: &[Vec<char>]) -> Option<(usize, usize)> {
    lines
        .iter()
        .enumerate()
        .find_map(|(r, l)| l.iter().position(|&ch| ch == 'S').map(|c| (r, c)))
}

fn count_splits(grid: &[Vec<char>], start: (usize, usize)) -> usize {
    let width = grid[0].len();
    let mut current = vec![false; width];
    let mut next = vec![false; width];
    current[start.1] = true;

    let mut splits = 0;
    for row in grid.iter().skip(start.0) {
        for c in 0..width {
            if !current[c] {
                continue;
            }

            match row[c] {
                '^' => {
                    splits += 1;
                    if c > 0 {
                        next[c - 1] = true;
                    }

                    if c + 1 < width {
                        next[c + 1] = true;
                    }
                }
                _ => {
                    next[c] = true;
                }
            }
        }

        current.fill(false);
        std::mem::swap(&mut current, &mut next);

        if !current.iter().any(|&b| b) {
            break;
        }

        next.fill(false);
    }

    splits
}

fn count_timelines(grid: &[Vec<char>], start: (usize, usize)) -> u128 {
    let width = grid[0].len();
    let mut current = vec![0u128; width];
    let mut next = vec![0u128; width];
    current[start.1] = 1;

    let mut exits: u128 = 0;

    for row in grid.iter().skip(start.0) {
        for c in 0..width {
            let branches = current[c];
            if branches == 0 {
                continue;
            }

            match row[c] {
                '^' => {
                    if c > 0 {
                        next[c - 1] = next[c - 1].saturating_add(branches);
                    } else {
                        exits = exits.saturating_add(branches);
                    }

                    if c + 1 < width {
                        next[c + 1] = next[c + 1].saturating_add(branches);
                    } else {
                        exits = exits.saturating_add(branches);
                    }
                }
                _ => {
                    next[c] = next[c].saturating_add(branches);
                }
            }
        }

        current.fill(0);
        std::mem::swap(&mut current, &mut next);
        next.fill(0);
    }

    exits + current.iter().sum::<u128>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_default()?;
    let lines: Vec<Vec<char>> = input.raw.lines().map(|l| l.chars().collect()).collect();
    let width = lines[0].len();
    let (start_row, start_col) = find_start(&lines).ok_or("no start")?;

    if lines.iter().any(|row| row.len() != width) {
        return Err("non-rectangular grid".into());
    }

    let part1 = count_splits(&lines, (start_row, start_col));
    println!("Part 1: {part1}");

    let part2 = count_timelines(&lines, (start_row, start_col));
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count_splits, count_timelines, find_start};

    const EXAMPLE: &str = concat!(
        ".......S.......\n",
        "...............\n",
        ".......^.......\n",
        "...............\n",
        "......^.^......\n",
        "...............\n",
        ".....^.^.^.....\n",
        "...............\n",
        "....^.^...^....\n",
        "...............\n",
        "...^.^...^.^...\n",
        "...............\n",
        "..^...^.....^..\n",
        "...............\n",
        ".^.^.^.^.^...^.\n",
        "...............",
    );

    #[test]
    fn splits_match_example() {
        let grid: Vec<Vec<char>> = EXAMPLE.lines().map(|l| l.chars().collect()).collect();
        let start = find_start(&grid).expect("start");
        assert_eq!(count_splits(&grid, start), 21);
    }

    #[test]
    fn timelines_match_example() {
        let grid: Vec<Vec<char>> = EXAMPLE.lines().map(|l| l.chars().collect()).collect();
        let start = find_start(&grid).expect("start");
        assert_eq!(count_timelines(&grid, start), 40);
    }
}
