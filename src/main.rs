use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

fn aoc1(w_size: usize) -> std::io::Result<()> {
    let mut f = File::open("d1.in")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let mut depths = buffer.lines().map(|l| l.parse::<i32>().unwrap());

    let mut windows: Vec<i32> = vec![];

    let mut to_be_summed: Vec<i32> = depths.by_ref().take(w_size).collect();
    if to_be_summed.len() < w_size {
        panic!("should not happen yet");
    }

    loop {
        windows.push(to_be_summed.iter().sum());
        to_be_summed.remove(0);
        match depths.next() {
            None => break,
            Some(x) => to_be_summed.push(x),
        }
    }
    let mut w = windows.into_iter();

    let start_val = (0, w.next().unwrap());

    let i = w.fold(start_val, |accum, v| {
        let (i, prev) = accum;
        if v > prev {
            (i + 1, v)
        } else {
            (i, v)
        }
    });
    println!("aoc1/{}: {}", w_size, i.0);
    Ok(())
}

fn main() -> std::io::Result<()> {
    aoc1(1)?;
    aoc1(3)?;
    Ok(())
}
