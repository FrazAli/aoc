use std::collections::HashMap;

fn decimal(snafu: String) -> i64 {
    let base: u64 = 5;
    let mut result: i64 = 0;
    let map: HashMap<char, i64> = [
        ('=', -2),
        ('-', -1),
        ('0', 0),
        ('1', 1),
        ('2', 2),
    ].iter().cloned().collect();
    for (i, c) in snafu.chars().rev().enumerate() {
        // println!("{} -> {}: {}", c, map[&c], base.pow(i as u32));
        let pwr: u64 = base.pow(i as u32) as u64;
        result += map[&c] * pwr as i64;
    }

    return result;
}

fn snafu(decimal: u64) -> String {
    let map = ["=", "-", "0", "1", "2"];
    let mut q: u64 = decimal;
    let mut r: u64;
    let mut result: String = "".to_string();
    while q > 0 {
        r = (q + 2) % 5;
        q = (q + 2) / 5;
        result += map[r as usize];
    }

    return result.chars().rev().collect::<String>();
}
fn main() {
    let readings: &str = include_str!("../input.txt");
    let mut part1: u64 = 0;
    for line in readings.lines() {
        // println!("Snafu #: {}", line);
        part1 += decimal(line.to_string()) as u64;
    }

    println!("part-1: {}", part1);
    println!("snafu: {}", snafu(part1));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        let snafu_inputs = vec![
            "1",
            "2",
            "1=",
            "1-",
            "10",
            "11",
            "12",
            "2=",
            "2-",
            "20",
            "1=0",
            "1-0",
            "1=11-2",
            "1-0---0",
            "1121-1110-1=0"
        ];
        let expected = vec![
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            15,
            20,
            2022,
            12345,
            314159265
        ];
        for (i, v) in snafu_inputs.into_iter().enumerate() {
            assert_eq!(decimal(v.to_string()), expected[i]);
        }
    }

    #[test]
    fn test_decimal_sum() {
        let snafu_inputs = vec![
            "1=-0-2",
            "12111",
            "2=0=",
            "21",
            "2=01",
            "111",
            "20012",
            "112",
            "1=-1=",
            "1-12",
            "12",
            "1=",
            "122",
        ];

        let mut result: i64 = 0;
        for (_, v) in snafu_inputs.into_iter().enumerate() {
            result += decimal(v.to_string());
        }
        assert_eq!(result, 4890);
    }

    #[test]
    fn test_snafu() {
        let expected = vec![
            "1",
            "2",
            "1=",
            "1-",
            "10",
            "11",
            "12",
            "2=",
            "2-",
            "20",
            "1=0",
            "1-0",
            "1=11-2",
            "1-0---0",
            "1121-1110-1=0"
        ];
        let decimal_inputs = vec![
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            15,
            20,
            2022,
            12345,
            314159265
        ];
        for (i, v) in decimal_inputs.into_iter().enumerate() {
            assert_eq!(snafu(v), expected[i]);
        }
    }
}

