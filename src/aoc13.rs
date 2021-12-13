use std::fs::read_to_string;

fn debug(nums: &Vec<Vec<u8>>) -> usize {
    let mut s = String::new();

    let mut pop_count = 0;
    for row in nums {
        for v in row {
            if *v == 1 {
                pop_count += 1;
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    if nums.len() <= 80 {
        println!("{}", s);
    }
    pop_count
}

fn fold_along_y(nums: &mut Vec<Vec<u8>>, y: &str) {
    let anchor = y.parse::<usize>().unwrap();
    println!("y fold={}", anchor);
    for i in 1..=anchor {
        if anchor + 1 == nums.len() {
            break;
        }
        if i > anchor {
            break;
        }
        for j in 0..nums[anchor - i].len() {
            nums[anchor - i][j] |= nums[anchor + 1][j];
        }
        nums.remove(anchor + 1);
    }
    nums.remove(anchor);
    if anchor >= nums.len() {
        for i in anchor + 1..nums.len() {
            let v = nums.remove(i);
            nums.insert(0, v);
        }
    }
}

fn fold_along_x(nums: &mut Vec<Vec<u8>>, x: &str) {
    let anchor = x.parse::<usize>().unwrap();
    println!("x fold={}", anchor);
    for i in 1..=anchor {
        if anchor + 1 == nums[0].len() {
            break;
        }
        if i > anchor {
            break;
        }
        for n in nums.into_iter() {
            // dbg!(anchor - i, anchor + 1);
            n[anchor - i] |= n[anchor + 1];
            n.remove(anchor + 1);
        }
    }
    for n in nums.into_iter() {
        n.remove(anchor);
    }
    if anchor >= nums.len() {
        for i in anchor + 1..nums.len() {
            for n in nums.into_iter() {
                let v = n.remove(i);
                n.insert(0, v);
            }
        }
    }
}

fn aoc(input: &str) -> usize {
    let mut nums = Vec::<Vec<u8>>::new();
    let mut max_x = 0;
    let mut parsing_coordinates = true;
    for l in input.lines() {
        if l.is_empty() {
            parsing_coordinates = false;
            for n in &mut nums {
                while max_x >= n.len() {
                    n.push(0);
                }
            }
            debug(&nums);
            continue;
        }
        if parsing_coordinates {
            let mut s = l.split(',').map(|x| x.parse::<usize>().unwrap());
            let x = s.next().unwrap();
            let y = s.next().unwrap();
            while y >= nums.len() {
                nums.push(Vec::<u8>::new());
            }
            if x > max_x {
                max_x = x;
            }
            while max_x >= nums[y].len() {
                nums[y].push(0);
            }
            nums[y][x] = 1;
        } else {
            match l.strip_prefix("fold along y=") {
                None => match l.strip_prefix("fold along x=") {
                    None => panic!("unexpected line {}", l),
                    Some(x) => fold_along_x(&mut nums, x),
                },
                Some(y) => fold_along_y(&mut nums, y),
            }
            debug(&nums);
        }
    }
    debug(&nums)
}
fn main() -> std::io::Result<()> {
    let test = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let res = aoc(test);
    assert_eq!(res, 16);
    println!("test done");

    let buffer = read_to_string(String::from("d13.in"))?;
    let res = aoc(&buffer);
    // ####.#..#...##.#..#..##..####.#..#.###..
    // ...#.#..#....#.#..#.#..#.#....#..#.#..#.
    // ..#..#..#....#.#..#.#..#.###..####.#..#.
    // .#...#..#....#.#..#.####.#....#..#.###..
    // #....#..#.#..#.#..#.#..#.#....#..#.#....
    // ####..##...##...##..#..#.#....#..#.#....
    // zujuafhp
    assert_eq!(res, 96);
    Ok(())
}
