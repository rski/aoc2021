use std::{collections::HashMap, fs::read_to_string};

fn produce(first_day: i64, mut cache: &mut HashMap<i64, usize>) -> usize {
    let mut i = 1;
    let mut fishes = vec![];
    loop {
        let dob = i * 7 + first_day;
        if dob > 256 {
            break;
        }
        i += 1;
        fishes.push(dob + 2)
    }
    fishes
        .iter()
        .fold(0, |acc, &f| acc + reproduce(f, &mut cache))
        + i as usize
        - 1
}

fn reproduce(first_day: i64, mut cache: &mut HashMap<i64, usize>) -> usize {
    match cache.get(&first_day) {
        Some(i) => *i,
        None => {
            let i = produce(first_day, &mut cache);
            cache.insert(first_day, i);
            i
        }
    }
}

fn main() -> std::io::Result<()> {
    let buffer = read_to_string(String::from("d6.in"))?;
    let state: Vec<i64> = buffer
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap() - 6)
        .collect();
    println!("Initial state: {:?}", state);

    let mut cache = HashMap::<i64, usize>::new();
    let num_fishes: usize = state.len()
        + state
            .iter()
            .map(|&f| reproduce(f, &mut cache))
            .sum::<usize>();
    println!("{} fish now", num_fishes);
    Ok(())
}
