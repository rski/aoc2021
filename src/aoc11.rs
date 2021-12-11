#[derive(Clone, Debug)]
struct Oct {
    v: u8,
    exploded: bool,
}
fn aoc(s: &str, steps: usize) -> u64 {
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

    let mut exploded: u64 = 0;
    for _ in 0..steps {
        exploded += take_step(&mut nums, line_len);
    }

    dbg!(exploded);
    exploded
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
                inc_neigh((i, j), &mut nums);
            }
        }
    }
    let mut exploded = 0;
    for i in 1..nums.len() - 1 {
        for j in 1..line_len - 1 {
            if nums[i][j].exploded {
                exploded += 1;
                nums[i][j].v = nums[i][j].v / 10;
                nums[i][j].exploded = false
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
        // (c.0, c.1), self
        (c.0, c.1 + 1),
        (c.0 + 1, c.1 - 1),
        (c.0 + 1, c.1),
        (c.0 + 1, c.1 + 1),
    ];
    for n in neighbors {
        let o = &mut points[n.0][n.1];
        if o.v < 19 {
            o.v += 1;
        }
        if o.exploded {
            return;
        }
        if o.v == 10 {
            o.exploded = true;
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
    let flashes = aoc(test, 10);
    assert_eq!(flashes, 204);
    let flashes = aoc(test, 100);
    assert_eq!(flashes, 1656);
    // let buffer = read_to_string(String::from("d10.in"))?;
    // let (score, ac) = aoc(buffer.trim());
    // assert_eq!(score, 315693);
    // assert_eq!(ac, 1870887234);
    Ok(())
}
