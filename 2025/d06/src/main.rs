use std::io::{Error, ErrorKind};

use common::input::Input;

fn validate_grid(lines: &[&str]) -> Result<usize, Box<dyn std::error::Error>> {
    if lines.is_empty() {
        return Err(Box::new(Error::new(ErrorKind::InvalidData, "empty input")));
    }

    let cols = lines[0].len();
    if let Some((i, line)) = lines.iter().enumerate().find(|(_, l)| l.len() != cols) {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidData,
            format!("line {i} has length {} instead of {cols}", line.len()),
        )));
    }

    Ok(cols)
}

fn find_spans(lines: &[&str], cols: usize) -> Vec<(usize, usize)> {
    let rows = lines.len();
    let mut sep_cols: Vec<usize> = Vec::new();
    for c in 0..cols {
        if (0..rows).all(|r| lines[r].as_bytes()[c] == b' ') {
            sep_cols.push(c);
        }
    }

    let mut spans = Vec::new();
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

    spans
}

fn slice_problems<'a>(lines: &'a [&str], spans: &[(usize, usize)]) -> Vec<Vec<&'a str>> {
    spans
        .iter()
        .map(|&(s, e)| lines.iter().map(|l| &l[s..=e]).collect())
        .collect()
}

fn operator(problem: &[&str]) -> Result<(char, usize), Box<dyn std::error::Error>> {
    let op_row = problem.len() - 1;
    let op_char = problem[op_row]
        .chars()
        .find(|&c| c != ' ')
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing operator"))?;
    Ok((op_char, op_row))
}

fn eval_part1(problem: &[&str]) -> Result<i64, Box<dyn std::error::Error>> {
    let (op_char, op_row) = operator(problem)?;
    let mut operands: Vec<i64> = Vec::new();
    for row in &problem[..op_row] {
        for tok in row.split_whitespace() {
            operands.push(tok.parse()?);
        }
    }

    let result = match op_char {
        '+' => operands.iter().sum::<i64>(),
        '*' => operands.iter().product::<i64>(),
        _ => {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidData,
                "invalid operator",
            )));
        }
    };

    Ok(result)
}

fn eval_part2(problem: &[&str]) -> Result<i64, Box<dyn std::error::Error>> {
    let (op_char, op_row) = operator(problem)?;
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

    let result = match op_char {
        '+' => operands.iter().sum::<i64>(),
        '*' => operands.iter().product::<i64>(),
        _ => {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidData,
                "invalid operator",
            )));
        }
    };

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_default()?;
    let lines: Vec<&str> = input.raw.lines().collect();
    let cols = validate_grid(&lines)?;
    let spans = find_spans(&lines, cols);
    let problems = slice_problems(&lines, &spans);

    let part1: i64 = problems
        .iter()
        .map(|p| eval_part1(p))
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?
        .into_iter()
        .sum();
    println!("Part 1: {part1}");

    let part2: i64 = problems
        .iter()
        .map(|p| eval_part2(p))
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?
        .into_iter()
        .sum();
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 4] = [
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    fn totals(lines: &[&str]) -> (i64, i64) {
        let cols = validate_grid(lines).unwrap();
        let spans = find_spans(lines, cols);
        let problems = slice_problems(lines, &spans);
        let p1: i64 = problems
            .iter()
            .map(|p| eval_part1(p))
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()
            .unwrap()
            .into_iter()
            .sum();
        let p2: i64 = problems
            .iter()
            .map(|p| eval_part2(p))
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()
            .unwrap()
            .into_iter()
            .sum();
        (p1, p2)
    }

    #[test]
    fn sample_part1() {
        let (p1, _) = totals(&EXAMPLE);
        assert_eq!(p1, 4_277_556);
    }

    #[test]
    fn sample_part2() {
        let (_, p2) = totals(&EXAMPLE);
        assert_eq!(p2, 3_263_827);
    }
}
