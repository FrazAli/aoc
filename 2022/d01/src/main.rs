use std::fs::File;
use std::io::Read;

struct Elf {
    calories: Vec<usize>,
    total: usize,
}

fn main() {
    let mut file: File = File::open("input.txt")
        .expect("Unable to open the file");
    let mut data: String = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read the file");

    let mut elves: Vec<Elf> = Vec::new();
    let mut values: Vec<usize> = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            elves.push(Elf {
                calories: values.clone(),
                total: values.iter().sum(),
            });
            values.truncate(0);  // drop all elements
        } else {
            values.push(line.parse().unwrap());
        }
    }

    elves.sort_by_key(|e| e.total);
    let elf: &Elf = &elves[elves.len() - 1];
    println!("Calories: {:?}", elf.calories);
    println!("Part-1: {}", elf.total);

    elves.reverse();
    elves.truncate(3);
    println!("Part-2: {}", elves.iter().map(|e| e.total).sum::<usize>());
}

