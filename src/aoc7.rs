use std::fs::read_to_string;

fn aoc(positions: &str) {
    let mut max = 0;
    let mut min = u64::MAX;
    let nums: Vec<u64> = positions
        .split(",")
        .map(|x| {
            let x = x.parse::<u64>().unwrap();
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
            x
        })
        .collect();
    let mut min_fuel = u64::MAX;
    for i in min..max {
        let mut fuel = 0;
        for n in &nums {
            if i == *n {
                continue;
            }
            let s = n.max(&i) - n.min(&i);
            fuel += (s * (s + 1)) / 2;
        }
        if fuel < min_fuel {
            min_fuel = fuel
        }
    }
    println!("{},{} cost: {}", min, max, min_fuel)
}

fn main() -> std::io::Result<()> {
    let test = "16,1,2,0,4,2,7,1,2,14";
    aoc(test);
    let buffer = read_to_string(String::from("d7.in"))?;
    aoc(buffer.trim());
    Ok(())
}
