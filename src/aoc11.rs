#[derive(Clone, Debug)]
struct Oct {
    v: u8,
    exploded: bool,
}

fn setup(s: &str) -> Vec<Vec<Oct>> {
    let h_val = Oct {
        v: 0,
        exploded: true,
    };
    let mut nums = Vec::<Vec<Oct>>::new();
    for l in s.lines() {
        let mut line_vec: Vec<Oct> = l
            .trim()
            .chars()
            .map(|x| Oct {
                v: x.to_digit(10).unwrap() as u8,
                exploded: false,
            })
            .collect();
        line_vec.insert(0_usize, h_val.clone());
        line_vec.push(h_val.clone());
        nums.push(line_vec);
    }
    let line_len = nums[0].len();
    let halo: Vec<Oct> = vec![h_val; line_len];
    nums.insert(0, halo.clone());
    nums.push(halo);
    nums
}

fn aoc(s: &str, steps: usize) -> u64 {
    let mut nums = setup(s);
    let mut exploded: u64 = 0;
    let line_len = nums[0].len();
    for _ in 0..steps {
        exploded += take_step(&mut nums, line_len);
    }
    exploded
}

fn aoc2(s: &str) -> u64 {
    let mut nums = setup(s);
    let line_len = nums[0].len();
    let size = (nums.len() - 2) * (line_len - 2);
    let mut i = 0;
    loop {
        i += 1;
        if size as u64 == take_step(&mut nums, line_len) {
            return i;
        }
    }
}

fn debug(nums: &Vec<Vec<Oct>>, line_len: usize) {
    for i in 1..nums.len() - 1 {
        for j in 1..line_len - 1 {
            print!("({:02}, {}) ", nums[i][j].v, nums[i][j].exploded as i32);
        }
        println!();
    }
    println!();
    for i in 1..nums.len() - 1 {
        for j in 1..line_len - 1 {
            print!("{}", nums[i][j].v);
        }
        println!();
    }
    println!();
}

fn take_step(mut nums: &mut Vec<Vec<Oct>>, line_len: usize) -> u64 {
    for i in 1..nums.len() - 1 {
        for j in 1..line_len - 1 {
            nums[i][j].v += 1
        }
    }
    for i in 1..nums.len() - 1 {
        for j in 1..line_len - 1 {
            if nums[i][j].v == 10 {
                nums[i][j].v = 0;
                nums[i][j].exploded = true;
                inc_neigh((i, j), &mut nums);
            }
        }
    }
    debug(&nums, line_len);
    let mut exploded = 0;
    for i in 1..nums.len() - 1 {
        for j in 1..line_len - 1 {
            if nums[i][j].exploded {
                exploded += 1;
                nums[i][j].exploded = false;
            }
        }
    }
    exploded
}

fn inc_neigh(c: (usize, usize), points: &mut Vec<Vec<Oct>>) {
    let neighbors = [
        (c.0 - 1, c.1 - 1),
        (c.0 - 1, c.1),
        (c.0 - 1, c.1 + 1),
        (c.0, c.1 - 1),
        (c.0, c.1 + 1),
        (c.0 + 1, c.1 - 1),
        (c.0 + 1, c.1),
        (c.0 + 1, c.1 + 1),
    ];
    for n in neighbors {
        let o = &mut points[n.0][n.1];
        if o.exploded {
            continue;
        }
        o.v += 1;
        if o.v >= 10 {
            o.exploded = true;
            o.v = 0;
            inc_neigh(n, points);
        }
    }
}

fn main() -> std::io::Result<()> {
    let test = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
    let flashes = aoc(test, 1);
    assert_eq!(flashes, 0);
    let flashes = aoc(test, 2);
    assert_eq!(flashes, 35);
    let flashes = aoc(test, 3);
    assert_eq!(flashes, 80);
    let flashes = aoc(test, 10);
    assert_eq!(flashes, 204);
    let flashes = aoc(test, 100);
    assert_eq!(flashes, 1656);

    let input = "\
4525436417
1851242553
5421435521
8431325447
4517438332
3521262111
3331541734
4351836641
2753881442
7717616863
";

    let flashes = aoc(input, 100);
    assert_eq!(flashes, 1652);

    let ss = aoc2(test);
    assert_eq!(ss, 195);
    let ss = aoc2(input);
    assert_eq!(ss, 220);
    Ok(())
}
