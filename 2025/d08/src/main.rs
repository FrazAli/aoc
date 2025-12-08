use std::collections::HashSet;
use std::error::Error;

use common::input::Input;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn parse(input: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut points = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
        if tokens.len() != 3 {
            return Err(format!("Invalid input {line}").into());
        }

        let x: i64 = tokens[0].parse()?;
        let y: i64 = tokens[1].parse()?;
        let z: i64 = tokens[2].parse()?;

        points.push(Point { x, y, z });
    }

    Ok(points)
}

fn distance_squared(a: &Point, b: &Point) -> i128 {
    let dx = (a.x - b.x) as i128;
    let dy = (a.y - b.y) as i128;
    let dz = (a.z - b.z) as i128;
    dx * dx + dy * dy + dz * dz
}

fn solve_part1(points: &[Point], max_connections: usize) -> usize {
    if points.len() < 3 {
        return 0;
    }

    /* Pre-allocate and build all point connections with squared distance.
    Unique pairs for n points: n*(n-1)/2 */
    let mut connections = Vec::with_capacity(points.len() * (points.len() - 1) / 2);
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            connections.push((distance_squared(&points[i], &points[j]), i, j));
        }
    }

    connections.sort_by(|a, b| a.0.cmp(&b.0));

    let mut circuits: Vec<HashSet<usize>> = (0..points.len())
        .map(|i| {
            let mut set = HashSet::new();
            set.insert(i);
            set
        })
        .collect();

    for &(_, a, b) in connections.iter().take(max_connections) {
        let mut idx_a = None;
        let mut idx_b = None;
        for (ci, set) in circuits.iter().enumerate() {
            if idx_a.is_none() && set.contains(&a) {
                idx_a = Some(ci);
            }
            if idx_b.is_none() && set.contains(&b) {
                idx_b = Some(ci);
            }
            if idx_a.is_some() && idx_b.is_some() {
                break;
            }
        }

        let ci = idx_a.expect("component for a not found");
        let cj = idx_b.expect("component for b not found");
        if ci == cj {
            continue; // already connected
        }

        // Merge smaller into larger to keep merge cost lower.
        let (mut large, mut small) = if circuits[ci].len() >= circuits[cj].len() {
            (ci, cj)
        } else {
            (cj, ci)
        };

        if small > large {
            std::mem::swap(&mut small, &mut large);
        }

        let to_merge = circuits[small].clone();
        circuits[large].extend(to_merge);
        circuits.swap_remove(small);
    }

    let mut sizes: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    // The product of top 3 circuit-sizes is the answer.
    sizes[0] * sizes[1] * sizes[2]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_default()?;
    let points = parse(&input.raw)?;

    let part1 = solve_part1(&points, 1000);
    println!("Part 1: {part1}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = concat!(
            "162,817,812\n",
            "57,618,57\n",
            "906,360,560\n",
            "592,479,940\n",
            "352,342,300\n",
            "466,668,158\n",
            "542,29,236\n",
            "431,825,988\n",
            "739,650,466\n",
            "52,470,668\n",
            "216,146,977\n",
            "819,987,18\n",
            "117,168,530\n",
            "805,96,715\n",
            "346,949,466\n",
            "970,615,88\n",
            "941,993,340\n",
            "862,61,35\n",
            "984,92,344\n",
            "425,690,689\n",
        );
        let points = match parse(input) {
            Ok(p) => p,
            Err(e) => panic!("failed to parse example: {e}"),
        };
        let result = solve_part1(&points, 10);
        assert_eq!(result, 40);
    }
}
