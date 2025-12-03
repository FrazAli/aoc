use std::io::Result;

use common::input::Input;

fn main() -> Result<()> {
    let input = Input::from_default()?;

    let part1 = joltage(&input.raw, 2);
    println!("Part 1: {part1}");

    let part2 = joltage(&input.raw, 12);
    println!("Part 2: {part2}");

    Ok(())
}

fn joltage(raw: &str, k: usize) -> u128 {
    raw.lines().map(|line| max_k_digit(line, k)).sum()
}

fn max_k_digit(line: &str, k: usize) -> u128 {
    let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();

    let mut stack: Vec<u8> = Vec::with_capacity(k);
    for (i, &d) in digits.iter().enumerate() {
        let remaining = digits.len() - i - 1;
        while let Some(&last) = stack.last() {
            if last < d && stack.len() + remaining >= k {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.len() < k {
            stack.push(d);
        }
    }

    stack.iter().fold(0u128, |acc, &d| acc * 10 + d as u128)
}

#[cfg(test)]
mod tests {
    use super::{joltage, max_k_digit};

    #[test]
    fn bank_examples() {
        assert_eq!(max_k_digit("987654321111111", 2), 98);
        assert_eq!(max_k_digit("811111111111119", 2), 89);
        assert_eq!(max_k_digit("234234234234278", 2), 78);
        assert_eq!(max_k_digit("818181911112111", 2), 92);
    }

    #[test]
    fn total_example() {
        let example_input = concat!(
            "987654321111111\n",
            "811111111111119\n",
            "234234234234278\n",
            "818181911112111",
        );
        assert_eq!(joltage(example_input, 2), 357);
        assert_eq!(joltage(example_input, 12), 3_121_910_778_619u128);
    }
}
