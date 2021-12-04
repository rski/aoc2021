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

fn get_numbers(b: &String) -> (Vec<u64>, u64) {
    let mut numbers: Vec<u64> = vec![];
    let mut lines = 0;
    let mut width = None;
    for l in b.lines() {
        let mut bp = Vec::<u8>::new();
        lines += 1;
        for c in l.chars() {
            match c {
                '0' => {
                    bp.push(0_u8);
                }
                '1' => {
                    bp.push(1_u8);
                }
                '\n' => {}
                _ => panic!("unexpected"),
            };
        }
        let mut a = 0_u64;
        for (i, v) in bp.iter().rev().enumerate() {
            a |= (*v as u64) << i;
        }
        width = Some(bp.len());
        numbers.push(a);
    }
    (numbers, width.unwrap() as u64)
}

fn db(n: &Vec<u64>) {
    for i in n.iter() {
        print!("{:08b}/{} ", i, i);
    }
    println!()
}

fn recursive_filter(numbers: &Vec<u64>, width: u64, flip: bool) -> u64 {
    let mut n = numbers.to_vec();
    for i in 0..width {
        let pos = width - i - 1;
        let pop_bit_pattern = most_popular_bits_pattern(&n, width); // this is calculated a bunch of types for initial n, but eh
        let mut v = pop_bit_pattern[i as usize];
        if flip {
            v = !v;
        }
        let s: u64 = v.into();
        dbg!(s);
        db(&n);
        n = n
            .iter()
            .filter(|&x| ((*x >> pos) & 1 == s))
            .map(|&x| x)
            .collect();
        db(&n);
        if n.len() <= 1 {
            break;
        }
    }

    if n.len() != 1 {
        panic!("wanted 1 number left, {:?} ", n)
    }

    n.pop().unwrap()
}

fn most_popular_bits_pattern(nums: &Vec<u64>, width: u64) -> Vec<bool> {
    let mut vals: Vec<u64> = vec![];

    for i in 0..width {
        vals.push(0);
        for n in nums {
            vals[i as usize] += (n >> i) & 1
        }
    }
    let s = nums.len();
    vals.iter().rev().map(|x| x * 2 >= s as u64).collect()
}

fn aoc3() -> std::io::Result<()> {
    let mut f = File::open("d3.in")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let (numbers, width) = get_numbers(&buffer);
    let vals = most_popular_bits_pattern(&numbers, width);

    let mut gamma = 0_u64;
    let mut epsilon = 0_u64;
    for (i, v) in vals.iter().rev().enumerate() {
        let v: u64 = (*v).into();
        gamma |= v << i;
        epsilon |= (v ^ 1) << i;
    }

    println!("{}, {}, {}", gamma, epsilon, gamma * epsilon);
    println!("{:?}", numbers);

    let oxy = recursive_filter(&numbers, width, false);
    let co2 = recursive_filter(&numbers, width, true);
    dbg!(oxy, co2, oxy * co2);
    Ok(())
}

fn main() -> std::io::Result<()> {
    aoc1(1)?;
    aoc1(3)?;
    aoc2()?;
    aoc3()?;
    Ok(())
}
