use std::{collections::HashMap, fs::read_to_string};

type Fish = i64;

fn reproduce(first_day: i64) -> Vec<i64> {
    let mut i = 1;
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
}

fn main() -> std::io::Result<()> {
    let buffer = read_to_string(String::from("d6.in"))?;
    let mut state: Vec<Fish> = buffer
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|x| x.parse::<Fish>().unwrap() - 6)
        .collect();
    println!("Initial state: {:?}", state);

    let mut num_fishes = state.len();
    let mut cache = HashMap::<i64, Vec<Fish>>::new();
    let (mut cache_hits, mut cache_inserts) = (0, 0);
    loop {
        let mut new_fish: Vec<Fish> = vec![];
        if state.len() == 0 {
            break;
        }
        for f in state {
            let cached = cache.get(&f);
            let mut this_fish = match cached {
                None => {
                    let new = reproduce(f);
                    cache_inserts += 1;
                    cache.insert(f, new.to_owned());
                    new
                }
                Some(x) => {
                    cache_hits += 1;
                    x.clone()
                }
            };
            num_fishes += this_fish.len();
            new_fish.append(&mut this_fish);
        }
        state = new_fish;
    }
    println!(
        "{} fish now ({} hits, {} inserts)",
        num_fishes, cache_hits, cache_inserts
    );
    Ok(())
}
