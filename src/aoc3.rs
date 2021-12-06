use std::fs::read_to_string;

fn get_numbers(b: &String) -> (Vec<u64>, u64) {
    let mut numbers: Vec<u64> = vec![];
    let mut width = None;
    for l in b.lines() {
        let mut bp = Vec::<u8>::new();
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

fn main() -> std::io::Result<()> {
    let buffer = read_to_string(String::from("d3.in"))?;
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
