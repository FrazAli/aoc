use std::collections::HashMap;

fn frequency_map(s: &str) -> HashMap<char, i64> {
    let mut fmap = HashMap::new();
    for c in s.chars() {
        *fmap.entry(c).or_insert(0) += 1;
    }

    return fmap;
}

fn priority(c: char) -> i64 {
    if c.is_lowercase() {
        (c as u8 - b'a') as i64 + 1
    } else {
        (c as u8 - b'A') as i64 + 27
    }
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    let input: &str = include_str!("../input.txt");
    // let input: &str = include_str!("../sample-input.txt");
    for line in input.lines() {
        // println!("Rucksack: {}", line);
        let c1 = &line[..line.len()/2];
        let c2 = &line[line.len()/2..];
        // println!("  Compartment-1: {}", c1);
        // println!("  Compartment-2: {}", c2);
        let c1_freq = frequency_map(c1);
        let c2_freq = frequency_map(c2);
        for (item, _) in c1_freq {
            if c2_freq.contains_key(&item) {
                // println!("{} {}", item, priority(item));
                part1 += priority(item);
            }
        }
    }

    for i in (0..input.lines().count()).step_by(3) {
        let lines = input.lines().collect::<Vec<_>>();
        let group = &lines[i..i+3];
        let mut gfreq = HashMap::new();
        for rucksack in group {
            let rfreq = frequency_map(rucksack);
            for (item, _) in rfreq {
                *gfreq.entry(item).or_insert(0) += 1;
            }
        }

        for (item, count) in gfreq {
            if count == 3 {
                part2 += priority(item);
            }
        }
    }

    println!("part-1 {}", part1);
    println!("part-2 {}", part2);
}
