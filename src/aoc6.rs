use std::{collections::HashMap, fs::read_to_string};

type Fish = i64;

fn produce(first_day: i64) -> Vec<Fish> {
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
}

fn reproduce(first_day: i64, mut cache: &mut HashMap<i64, usize>) -> usize {
    let i = cache.get(&first_day);
    if i.is_some() {
        return *i.unwrap();
    }
    let x = produce(first_day);
    let i = x.len() + x.iter().map(|&f| reproduce(f, &mut cache)).sum::<usize>();
    cache.insert(first_day, i);
    i
}

fn main() -> std::io::Result<()> {
    let buffer = read_to_string(String::from("d6.in"))?;
    let state: Vec<Fish> = buffer
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|x| x.parse::<Fish>().unwrap() - 6)
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
