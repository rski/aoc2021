use std::{collections::HashMap, fs::read_to_string};

type Fish = i64;

fn reproduce(first_day: i64, mut cache: &mut HashMap<i64, Vec<Fish>>) -> usize {
    let mut i = 1;
    let mut produce = || {
        let mut fishes = vec![];
        loop {
            let dob = i * 7 + first_day;
            if dob > 80 {
                break;
            }
            i += 1;
            fishes.push(dob + 2)
        }
        fishes
    };
    let fishes = produce();
    let i: usize = fishes.iter().map(|&f| reproduce(f, &mut cache)).sum();
    fishes.len() + i
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

    let mut cache = HashMap::<i64, Vec<Fish>>::new();
    let num_fishes: usize = state.len()
        + state
            .iter()
            .map(|&f| reproduce(f, &mut cache))
            .sum::<usize>();
    println!("{} fish now", num_fishes);
    Ok(())
}
