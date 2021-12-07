use std::fs::read_to_string;

struct Fish(u64);

impl std::fmt::Debug for Fish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl Fish {
    fn new(age: u64) -> Fish {
        Fish(age)
    }

    fn pass_day(&mut self) -> Option<Fish> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Fish::new(8))
        } else {
            self.0 -= 1;
            None
        }
    }
}

fn main() -> std::io::Result<()> {
    let buffer = read_to_string(String::from("d6.in"))?;
    let mut state: Vec<Fish> = buffer
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|x| Fish(x.parse::<u64>().unwrap()))
        .collect();
    println!("Initial state: {:?}", state);

    for _ in 0..80 {
        let mut new_fish: Vec<Fish> = vec![];
        for f in &mut state {
            match f.pass_day() {
                None => {}
                Some(f) => new_fish.push(f),
            }
        }
        state.append(&mut new_fish);
    }
    println!("{} fish now", state.len());
    Ok(())
}
