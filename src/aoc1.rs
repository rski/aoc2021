use std::fs::read_to_string;

fn aoc1(w_size: usize) -> std::io::Result<()> {
    let buffer = read_to_string(String::from("d1.in"))?;
    let mut depths = buffer.lines().map(|l| l.parse::<i32>().unwrap());

    let mut window: Vec<i32> = depths.by_ref().take(w_size).collect();
    if window.len() < w_size {
        panic!("should not happen yet");
    }

    let mut prev_sum: i32 = window.iter().sum();
    let mut i = 0;
    for x in depths {
        window.remove(0);
        window.push(x);
        let s = window.iter().sum();
        if s > prev_sum {
            i += 1;
        }
        prev_sum = s
    }
    println!("aoc1/{}: {}", w_size, i);
    Ok(())
}

fn main() -> std::io::Result<()> {
    aoc1(1)?;
    aoc1(3)?;
    Ok(())
}
