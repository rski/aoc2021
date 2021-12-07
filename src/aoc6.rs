use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone)]
// The i64 is the day the fish's counteDay ready is the first day the fish's counter was 6. This is negative for initial state fish.
struct Fish(i64);

impl std::fmt::Debug for Fish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl Fish {
    fn reproduce(&self) -> Vec<Fish> {
        let mut i = 1;
        let mut fishes = vec![];
        loop {
            let dob = i * 7 + self.0;
            if dob > 80 {
                break;
            }
            i += 1;
            fishes.push(Fish(dob + 2))
        }
        fishes
    }
}

fn main() -> std::io::Result<()> {
    let buffer = read_to_string(String::from("d6.in"))?;
    let mut state: Vec<Fish> = buffer
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|x| Fish(x.parse::<i64>().unwrap() - 6))
        .collect();
    println!("Initial state: {:?}", state);

    let mut num_fishes = state.len();
    let mut cache = HashMap::<i64, Vec<Fish>>::new();
    loop {
        let mut new_fish: Vec<Fish> = vec![];
        if state.len() == 0 {
            break;
        }
        for f in state {
            let cached = cache.get(&f.0);
            let mut this_fish = match cached {
                None => {
                    let new = f.reproduce();
                    cache.insert(f.0, new.to_owned());
                    new
                }
                Some(x) => x.clone(),
            };
            num_fishes += this_fish.len();
            new_fish.append(&mut this_fish);
        }
        state = new_fish;
    }
    println!("{} fish now", num_fishes);
    Ok(())
}
