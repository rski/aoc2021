fn debug(nums: &Vec<Vec<u8>>) {
    let mut s = String::new();
    for row in nums {
        for v in row {
            if *v == 1 {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn fold_along_x(nums: &mut Vec<Vec<u8>>, x: &str) {
    println!("todo: x={}", x);
}

fn fold_along_y(nums: &mut Vec<Vec<u8>>, x: &str) {
    println!("todo: y={}", x);
}

fn aoc(input: &str) {
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
    debug(&nums);
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
    aoc(test);
    Ok(())
}
