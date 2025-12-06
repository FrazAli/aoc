use common::input::Input;

fn main() {
    let input = Input::from_default().expect("failed to read input");
    let ranges = parse_ranges(&input.raw);

    // print and check range sizes
    // print_range_sizes(&ranges);

    let max_end = ranges.iter().map(|&(_, end)| end).max().unwrap_or(0);

    let part1_ids = generate_repeated_ids(max_end, 2, Some(2));
    let part1: u128 = ranges
        .iter()
        .map(|&(start, end)| sum_ids_in_range(start, end, &part1_ids))
        .sum();
    println!("Part 1: {part1}");

    let part2_ids = generate_repeated_ids(max_end, 2, None);
    let part2: u128 = ranges
        .iter()
        .map(|&(start, end)| sum_ids_in_range(start, end, &part2_ids))
        .sum();
    println!("Part 2: {part2}");
}

fn parse_ranges(raw: &str) -> Vec<(u64, u64)> {
    raw.split(',')
        .filter_map(|chunk| {
            let trimmed = chunk.trim();
            if trimmed.is_empty() {
                return None;
            }

            let (a, b) = trimmed
                .split_once('-')
                .expect("range missing dash separator");
            let start: u64 = a.parse().expect("invalid start number");
            let end: u64 = b.parse().expect("invalid end number");
            Some((start, end))
        })
        .collect()
}

#[allow(dead_code)]
fn print_range_sizes(ranges: &[(u64, u64)]) {
    for &(start, end) in ranges {
        assert!(start <= end);
        println!("{}", end - start);
    }
}

fn generate_repeated_ids(max_value: u64, min_repeats: u32, max_repeats: Option<u32>) -> Vec<u64> {
    let mut ids = Vec::new();
    let max_digits = num_digits(max_value);

    for block_len in 1..=max_digits {
        let pow10_block = 10_u64.pow(block_len as u32);
        let min_block = pow10_block / 10; // no leading zeros
        let max_block = pow10_block - 1;

        let max_possible_repeats = (max_digits / block_len) as u32;
        let upper_repeats = max_repeats.unwrap_or(max_possible_repeats);
        let repeats_cap = upper_repeats.min(max_possible_repeats);

        if repeats_cap < min_repeats {
            continue;
        }

        for block in min_block..=max_block {
            for reps in min_repeats..=repeats_cap {
                let num = repeat_block(block, pow10_block, reps);
                if num > max_value as u128 {
                    break;
                }
                ids.push(num as u64);
            }
        }
    }

    ids.sort_unstable();
    ids.dedup();
    ids
}

fn num_digits(n: u64) -> usize {
    let mut digits = 1;
    let mut value = n;
    while value >= 10 {
        value /= 10;
        digits += 1;
    }
    digits
}

fn repeat_block(block: u64, base: u64, repeats: u32) -> u128 {
    let mut result: u128 = 0;
    let block128 = block as u128;
    let base128 = base as u128;
    for _ in 0..repeats {
        result = result * base128 + block128;
    }
    result
}

fn sum_ids_in_range(start: u64, end: u64, ids: &[u64]) -> u128 {
    let lower = ids.partition_point(|&v| v < start);
    let upper = ids.partition_point(|&v| v <= end);
    ids[lower..upper].iter().map(|&v| v as u128).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sum_matches() {
        let input = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        let ranges = parse_ranges(input);
        let max_end = ranges.iter().map(|&(_, end)| end).max().unwrap();
        let ids = generate_repeated_ids(max_end, 2, Some(2));
        let total: u128 = ranges
            .iter()
            .map(|&(s, e)| sum_ids_in_range(s, e, &ids))
            .sum();
        assert_eq!(total, 1_227_775_554);
    }

    #[test]
    fn example_part2_matches() {
        let input = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        let ranges = parse_ranges(input);
        let max_end = ranges.iter().map(|&(_, end)| end).max().unwrap();
        let ids = generate_repeated_ids(max_end, 2, None);
        let total: u128 = ranges
            .iter()
            .map(|&(s, e)| sum_ids_in_range(s, e, &ids))
            .sum();
        assert_eq!(total, 4_174_379_265);
    }
}
