use std::collections::HashMap;

fn main() {
    let shape_score: HashMap<char, i64> = [
        ('X', 1),
        ('Y', 2),
        ('Z', 3),
    ].iter().cloned().collect();
    let shape: HashMap<char, char> = [
        ('A', 'R'),
        ('B', 'P'),
        ('C', 'S'),
        ('X', 'R'),
        ('Y', 'P'),
        ('Z', 'S'),
    ].iter().cloned().collect();
    let score = |a: char, b: char| -> i64 {
        if a == b {
            return 3;
        }

        if (a == 'R' && b =='S') || (a == 'S' && b == 'P') || (a == 'P' && b == 'R') {
            return 0;
        }

        return 6;
    };

    let strategized = |a: char, b: char| -> i64 {
        if a == 'A' && b == 'X' { return 3; }
        if a == 'A' && b == 'Y' { return 4; }
        if a == 'A' && b == 'Z' { return 8; }
        if a == 'B' && b == 'X' { return 1; }
        if a == 'B' && b == 'Y' { return 5; }
        if a == 'B' && b == 'Z' { return 9; }
        if a == 'C' && b == 'X' { return 2; }
        if a == 'C' && b == 'Y' { return 6; }
        if a == 'C' && b == 'Z' { return 7; }
        return 0;
    };

    let input: &str = include_str!("../input.txt");
    // let input: &str = include_str!("../sample-input.txt");
    let mut total_score: i64 = 0;
    let mut total_score_part2: i64 = 0;
    for line in input.lines() {

        let a: char = line.chars().nth(0).unwrap();
        let b: char = line.chars().nth(2).unwrap();
        let round_score: i64 = score(shape[&a], shape[&b]) + shape_score[&b];
        total_score += round_score;

        total_score_part2 += strategized(a, b);
    }

    println!("Part-1 #: {}", total_score);
    println!("Part-2 #: {}", total_score_part2);
}
