use common::input::Input;
use std::error::Error;

fn parse_moves(input: &Input) -> Result<Vec<(char, u32)>, Box<dyn Error>> {
    let mut moves = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (d, n) = line.split_at(1);
        let dir = d.chars().next().ok_or("No direction")?;
        let dist: u32 = n.parse().map_err(|_| "Not a number")?;
        moves.push((dir, dist));
    }

    Ok(moves)
}

fn count_zero_positions(moves: &[(char, u32)]) -> u32 {
    let mut count = 0;
    let mut pos: i32 = 50;
    for &(m, n) in moves {
        let next: i32 = match m {
            'R' => pos + n as i32,
            'L' => pos - n as i32,
            _ => panic!("Unknown direction"),
        };

        pos = next.rem_euclid(100);

        if pos == 0 {
            count += 1;
        }
    }

    count
}

fn count_zero_hits(moves: &[(char, u32)]) -> u32 {
    let mut count = 0;
    let mut pos: i32 = 50;
    for &(m, n) in moves {
        let mut steps_to_zero: i32 = match m {
            'R' => (100 - (pos % 100)) % 100,
            'L' => pos % 100,
            _ => panic!("Unknown direction"),
        };

        if steps_to_zero == 0 {
            steps_to_zero = 100;
        }

        if n >= steps_to_zero as u32 {
            count += 1 + ((n - steps_to_zero as u32) / 100);
        }

        let next = match m {
            'R' => pos + n as i32,
            'L' => pos - n as i32,
            _ => panic!("Unknown direction"),
        };
        pos = next.rem_euclid(100);
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_default().unwrap_or(Input {
        raw: "".to_string(),
    });

    let moves = parse_moves(&input)?;

    let part1 = count_zero_positions(&moves);
    println!("Part 1: {}", part1);

    let part2 = count_zero_hits(&moves);
    println!("Part 2: {}", part2);

    Ok(())
}
