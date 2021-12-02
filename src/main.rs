use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

fn aoc1(w_size: usize) -> std::io::Result<()> {
    let mut f = File::open("d1.in")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let mut depths = buffer.lines().map(|l| l.parse::<i32>().unwrap());

    let mut window: Vec<i32> = depths.by_ref().take(w_size).collect();
    if window.len() < w_size {
        panic!("should not happen yet");
    }

    let mut prev_sum: i32 = window.iter().sum();
    let mut i = 0;
    while let Some(x) = depths.next() {
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

struct Subnautica {
    d: i32,
    fwd: i32,
    aim: i32,
    aim_depth: i32,
}

fn aoc2() -> std::io::Result<()> {
    let mut f = File::open("d2.in")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let directions = buffer.lines().map(|l| {
        let mut it = l.split_ascii_whitespace();
        let direction = it.next().unwrap();
        let v = it.next().unwrap().parse::<i32>().unwrap();
        (direction, v)
    });

    let mut s = Subnautica {
        d: 0,
        fwd: 0,
        aim: 0,
        aim_depth: 0,
    };
    for d in directions {
        match d {
            ("forward", x) => {
                s.fwd += x;
                s.aim_depth += s.aim * x;
            }
            ("down", x) => {
                s.d += x;
                s.aim += x;
            }
            ("up", x) => {
                s.d -= x;
                s.aim -= x;
            }
            (d, _) => panic!("unexpected direction {}", d),
        };
    }
    println!(
        "aoc2_p1: {}, aim_depth:{}",
        s.d * s.fwd,
        s.fwd * s.aim_depth
    );

    Ok(())
}

fn main() -> std::io::Result<()> {
    aoc1(1)?;
    aoc1(3)?;
    aoc2()?;
    Ok(())
}
