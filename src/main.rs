use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

fn aoc1(w_size: usize) -> std::io::Result<()> {
    let mut f = File::open("d1.in")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let mut depths = buffer.lines().map(|l| l.parse::<i32>().unwrap());

    let mut to_be_summed: Vec<i32> = depths.by_ref().take(w_size).collect();
    if to_be_summed.len() < w_size {
        panic!("should not happen yet");
    }

    let mut prev_sum: i32 = to_be_summed.iter().sum();
    let mut i = 0;
    loop {
        to_be_summed.remove(0);
        match depths.next() {
            None => break,
            Some(x) => to_be_summed.push(x),
        }
        let s = to_be_summed.iter().sum();
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
