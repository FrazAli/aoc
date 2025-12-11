use std::collections::VecDeque;
use z3::{Optimize, ast::Int};

use common::input::Input;

fn solve_part1(raw: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total_presses: i32 = 0;

    for line in raw.lines() {
        let machine = parse_machine(line)?;
        let presses = min_presses_bfs(machine.target_mask, machine.light_count, &machine.buttons)
            .ok_or("unsolvable machine configuration")?;
        total_presses += presses;
    }

    Ok(total_presses)
}

fn solve_part2(raw: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total_presses: i32 = 0;

    for line in raw.lines() {
        let machine = parse_machine(line)?;
        let presses = min_presses_counters(&machine.joltage, &machine.buttons)
            .ok_or("unsolvable machine configuration (part 2)")?;
        total_presses += presses;
    }

    Ok(total_presses)
}

struct Machine {
    target_mask: u16,
    light_count: usize,
    buttons: Vec<u16>,
    joltage: Vec<i32>,
}

fn parse_machine(line: &str) -> Result<Machine, Box<dyn std::error::Error>> {
    let start = line.find('[').ok_or("missing '[' in machine line")?;
    let end = line[start..]
        .find(']')
        .map(|idx| start + idx)
        .ok_or("missing ']' in machine line")?;

    let pattern = &line[start + 1..end];
    let light_count = pattern.len();

    let mut target_mask: u16 = 0;
    for (i, ch) in pattern.as_bytes().iter().enumerate() {
        if *ch == b'#' {
            target_mask |= 1 << i;
        }
    }

    let cut = line.find('{').unwrap_or(line.len());
    let mut buttons = Vec::new();
    let mut idx = end + 1;
    let bytes = line.as_bytes();
    while idx < cut {
        while idx < cut && bytes[idx] != b'(' {
            idx += 1;
        }

        if idx >= cut {
            break;
        }

        let start_paren = idx + 1;
        let mut end_paren = start_paren;
        while end_paren < cut && bytes[end_paren] != b')' {
            end_paren += 1;
        }

        if end_paren >= cut {
            break;
        }

        let content = &line[start_paren..end_paren];
        let mut mask: u16 = 0;
        if !content.trim().is_empty() {
            for num_str in content.split(',') {
                let pos: usize = num_str.trim().parse()?;
                if pos >= 16 {
                    return Err("button index exceeds 15".into());
                }
                mask ^= 1 << pos; // toggling same light twice, xor
            }
        }

        buttons.push(mask);
        idx = end_paren + 1;
    }

    // Parse joltage targets
    let mut joltage_targets = Vec::new();
    if let Some(open) = line.find('{')
        && let Some(close_rel) = line[open..].find('}')
    {
        let close = open + close_rel;
        let inner = &line[open + 1..close];
        joltage_targets = inner
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
    }

    Ok(Machine {
        target_mask,
        light_count,
        buttons,
        joltage: joltage_targets,
    })
}

/// BFS over light states to find minimum
fn min_presses_bfs(target: u16, light_count: usize, buttons: &[u16]) -> Option<i32> {
    let state_count = 1usize << light_count; // 2^light_count
    let mut steps = vec![-1i32; state_count];
    let mut q = VecDeque::new();

    steps[0] = 0;
    q.push_back(0usize);

    while let Some(state) = q.pop_front() {
        let current = steps[state];
        if state as u16 == target {
            return Some(current);
        }

        for &btn in buttons {
            // press button - xor to toggle bits
            let next = state ^ btn as usize;
            if steps[next] == -1 {
                steps[next] = current + 1;
                q.push_back(next);
            }
        }
    }

    None
}

/// Minimum presses to reach exact target counters using Z3
fn min_presses_counters(targets: &[i32], buttons: &[u16]) -> Option<i32> {
    if targets.is_empty() {
        return Some(0);
    }

    let optimizer = Optimize::new();

    let button_vars: Vec<Int> = buttons
        .iter()
        .enumerate()
        .map(|(i, _)| Int::new_const(format!("b{i}")))
        .collect();

    for var in &button_vars {
        optimizer.assert(&var.ge(Int::from_i64(0)));
    }

    for (counter_idx, &target) in targets.iter().enumerate() {
        let mut terms = Vec::new();
        for (button_idx, &mask) in buttons.iter().enumerate() {
            if ((mask >> counter_idx) & 1) == 1 {
                terms.push(button_vars[button_idx].clone());
            }
        }
        let sum = if terms.is_empty() {
            Int::from_i64(0)
        } else {
            Int::add(&terms.iter().collect::<Vec<_>>())
        };
        optimizer.assert(&sum.eq(Int::from_i64(target as i64)));
    }

    let total_presses = Int::add(&button_vars.iter().collect::<Vec<_>>());
    optimizer.minimize(&total_presses);

    // If we have a solution, convert and return it as i32, otherwise return None
    match optimizer.check(&[]) {
        z3::SatResult::Sat | z3::SatResult::Unknown => optimizer
            .get_model()
            .and_then(|model| model.eval(&total_presses, true))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
        z3::SatResult::Unsat => None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_default()?;

    let part1 = solve_part1(&input.raw)?;
    println!("Part 1: {part1}");

    let part2 = solve_part2(&input.raw)?;
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        concat!(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n",
        )
    }

    #[test]
    fn example_min_presses_part1() {
        let total = solve_part1(example_input()).expect("example should be solvable");
        assert_eq!(total, 7);
    }

    #[test]
    fn example_min_presses_part2() {
        let total = solve_part2(example_input()).expect("example should be solvable");
        assert_eq!(total, 33);
    }

    #[test]
    fn parse_machine_basic() {
        let line = "[.##.] (1,3) (0,2) {}";
        let m = parse_machine(line).expect("parse ok");
        assert_eq!(m.light_count, 4);
        assert_eq!(m.target_mask, 0b0110);
        assert_eq!(m.buttons.len(), 2);
        assert_eq!(m.buttons[0], (1u16 << 1) | (1u16 << 3));
        assert_eq!(m.buttons[1], (1u16 << 0) | (1u16 << 2));
        assert!(m.joltage.is_empty());
    }
}
