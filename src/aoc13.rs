fn debug(nums: &[u32], max_x: usize) {
    let mut s = String::new();
    for n in nums {
        for i in 0..=max_x {
            let v = (n >> i) & 0x1;
            if v == 1 {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn fold_along_x(nums: &mut Vec<u32>, x: &str) {
    println!("todo: x={}", x);
}
fn fold_along_y(nums: &mut Vec<u32>, x: &str) {
    println!("todo: y={}", x);
}

fn aoc(input: &str) {
    let mut nums = Vec::<u32>::new();
    let mut max_x = 0;
    let mut parsing_coordinates = true;
    for l in input.lines() {
        if l.is_empty() {
            parsing_coordinates = false;
            continue;
        }
        if parsing_coordinates {
            let mut s = l.split(',').map(|x| x.parse::<usize>().unwrap());
            let x = s.next().unwrap();
            let y = s.next().unwrap();
            while y >= nums.len() {
                nums.push(0);
            }
            if x > max_x {
                max_x = x;
            }
            nums[y] = nums[y] | (1 << x);
        } else {
            match l.strip_prefix("fold along y=") {
                None => match l.strip_prefix("fold along x=") {
                    None => panic!("unexpected line {}", l),
                    Some(x) => fold_along_x(&mut nums, x),
                },
                Some(y) => fold_along_y(&mut nums, y),
            }
        }
    }
    debug(&nums, max_x);
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
