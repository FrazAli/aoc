use std::io::Result;

use common::input::Input;

fn main() -> Result<()> {
    let input = Input::from_default()?;
    let (ranges, ids) = parse_input(&input.raw);
    let merged = merge_ranges(ranges);

    let part1 = count_fresh(&merged, &ids);
    println!("Part 1: {part1}");

    let part2 = total_fresh(&merged);
    println!("Part 2: {part2}");

    Ok(())
}

fn parse_input(raw: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut lines = raw.lines();
    let mut ranges = Vec::new();

    for line in lines.by_ref() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        let (start, end) = trimmed.split_once('-').expect("invalid range");
        let start: i64 = start.parse().expect("invalid start");
        let end: i64 = end.parse().expect("invalid end");
        ranges.push((start, end));
    }

    let ids = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<i64>().expect("invalid id"))
        .collect();

    (ranges, ids)
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by_key(|&(s, _)| s);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in ranges {
        if let Some((_, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
                continue;
            }
        }
        merged.push((start, end));
    }

    merged
}

fn is_fresh(id: i64, ranges: &[(i64, i64)]) -> bool {
    // ranges are merged and sorted, so a simple scan with early exit is fine.
    for &(start, end) in ranges {
        if id < start {
            return false;
        }
        if id <= end {
            return true;
        }
    }
    false
}

fn count_fresh(ranges: &[(i64, i64)], ids: &[i64]) -> usize {
    ids.iter().filter(|&&id| is_fresh(id, ranges)).count()
}

fn total_fresh(ranges: &[(i64, i64)]) -> i64 {
    ranges.iter().map(|(s, e)| e - s + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        concat!(
            "3-5\n", "10-14\n", "16-20\n", "12-18\n", "\n", "1\n", "5\n", "8\n", "11\n", "17\n",
            "32\n",
        )
    }

    #[test]
    fn example_fresh_count() {
        let (ranges, ids) = parse_input(example());
        let merged = merge_ranges(ranges);
        assert_eq!(count_fresh(&merged, &ids), 3);
    }

    #[test]
    fn example_total_fresh() {
        let (ranges, _) = parse_input(example());
        let merged = merge_ranges(ranges);
        assert_eq!(total_fresh(&merged), 14);
    }

    #[test]
    fn merge_overlapping_ranges() {
        let merged = merge_ranges(vec![(1, 3), (2, 5), (10, 12), (12, 15)]);
        assert_eq!(merged, vec![(1, 5), (10, 15)]);
    }
}
