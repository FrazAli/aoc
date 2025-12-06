use std::io::{Error, ErrorKind};

use common::input::Input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_default()?;
    let lines: Vec<&str> = input.raw.lines().collect();
    if lines.is_empty() {
        return Err(Box::new(Error::new(ErrorKind::InvalidData, "empty input")));
    }

    let cols = lines[0].len();
    let rows = lines.len();
    if let Some((i, line)) = lines.iter().enumerate().find(|(_, l)| l.len() != cols) {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidData,
            format!("line {i} has length {} instead of {cols}", line.len()),
        )));
    }

    let mut sep_cols: Vec<usize> = Vec::new();
    for c in 0..cols {
        let mut sep = true;
        for r in 0..rows {
            if lines[r].as_bytes()[c] != b' ' {
                sep = false;
                break;
            }
        }

        if sep {
            sep_cols.push(c);
        }
    }

    let mut spans: Vec<(usize, usize)> = Vec::new();
    let mut start = 0;
    for &sep in &sep_cols {
        if sep > start {
            spans.push((start, sep - 1));
        }

        start = sep + 1;
    }

    if start < cols {
        spans.push((start, cols - 1));
    }

    let mut problems: Vec<Vec<&str>> = Vec::new();
    for &(start, end) in &spans {
        problems.push(lines.iter().map(|l| &l[start..=end]).collect());
    }

    let mut part1: i64 = 0;
    for problem in &problems {
        let op_row = problem.len() - 1;
        let operator = problem[op_row]
            .chars()
            .find(|&c| c != ' ')
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing operator"))?;
        let mut operands: Vec<i64> = Vec::new();
        for span in &problem[..op_row] {
            for tok in span.split_whitespace() {
                operands.push(tok.parse()?);
            }
        }

        let result: i64 = match operator {
            '+' => operands.iter().sum(),
            '*' => operands.iter().product(),
            _ => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    "invalid operator",
                )));
            }
        };

        part1 += result;
    }
    println!("Part 1: {part1}");

    let mut part2: i64 = 0;
    for problem in &problems {
        let op_row = problem.len() - 1;
        let operator = problem[op_row]
            .chars()
            .find(|&c| c != ' ')
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing operator"))?;
        let width = problem[0].len();
        let mut operands: Vec<i64> = Vec::new();
        for c in 0..width {
            let mut digits = String::new();
            for r in 0..op_row {
                let ch = problem[r].as_bytes()[c] as char;
                if ch != ' ' {
                    digits.push(ch);
                }
            }
            if !digits.is_empty() {
                operands.push(digits.parse()?);
            }
        }

        let result: i64 = match operator {
            '+' => operands.iter().sum(),
            '*' => operands.iter().product(),
            _ => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    "invalid operator",
                )));
            }
        };

        part2 += result;
    }

    println!("Part 2: {part2}");

    Ok(())
}
