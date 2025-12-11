use std::collections::{HashMap, HashSet};
use std::io::{self, ErrorKind};

use common::input::Input;

fn parse(raw: &str) -> Result<HashMap<String, Vec<String>>, io::Error> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in raw.lines() {
        let (node, rest) = line.split_once(':').ok_or_else(|| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("Malformed line (missing ':'): {line}"),
            )
        })?;
        let dests: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(node.trim().to_string(), dests);
    }

    Ok(graph)
}

fn count_paths(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<(String, Vec<String>), u64>,
    visiting: &mut HashSet<String>,
    required: Vec<String>,
) -> u64 {
    if node == "out" {
        return if required.is_empty() { 1 } else { 0 };
    }

    let key = (node.to_string(), required.clone());
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    // handle cycles
    if !visiting.insert(node.to_string()) {
        return 0;
    }

    // Remove current node from required if it matches
    let mut next_required = required.clone();
    if let Some(pos) = required.iter().position(|r| r == node) {
        next_required.remove(pos);
    }

    let mut total = 0u64;
    if let Some(neighbors) = graph.get(node) {
        for next in neighbors {
            total += count_paths(next, graph, cache, visiting, next_required.clone());
        }
    }

    visiting.remove(node);
    cache.insert(key, total);
    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_default()?;
    let graph = parse(&input.raw)?;

    let mut cache1 = HashMap::new();
    let mut visiting1 = HashSet::new();
    let part1 = count_paths("you", &graph, &mut cache1, &mut visiting1, Vec::new());
    println!("Part 1: {part1}");

    let mut cache2 = HashMap::new();
    let mut visiting2 = HashSet::new();
    let required = vec!["dac".to_string(), "fft".to_string()];
    let part2 = count_paths("svr", &graph, &mut cache2, &mut visiting2, required);
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<(), Box<dyn std::error::Error>> {
        let raw = concat!(
            "aaa: you hhh\n",
            "you: bbb ccc\n",
            "bbb: ddd eee\n",
            "ccc: ddd eee fff\n",
            "ddd: ggg\n",
            "eee: out\n",
            "fff: out\n",
            "ggg: out\n",
            "hhh: ccc fff iii\n",
            "iii: out\n",
        );
        let graph = parse(raw)?;
        let mut cache = HashMap::new();
        let mut visiting = HashSet::new();
        let count = count_paths("you", &graph, &mut cache, &mut visiting, Vec::new());
        assert_eq!(count, 5);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn std::error::Error>> {
        let raw = concat!(
            "svr: aaa bbb\n",
            "aaa: fft\n",
            "fft: ccc\n",
            "bbb: tty\n",
            "tty: ccc\n",
            "ccc: ddd eee\n",
            "ddd: hub\n",
            "hub: fff\n",
            "eee: dac\n",
            "dac: fff\n",
            "fff: ggg hhh\n",
            "ggg: out\n",
            "hhh: out\n",
        );
        let graph = parse(raw)?;
        let mut cache = HashMap::new();
        let mut visiting = HashSet::new();
        let required = vec!["dac".to_string(), "fft".to_string()];
        let count = count_paths("svr", &graph, &mut cache, &mut visiting, required);
        assert_eq!(count, 2);
        Ok(())
    }
}
