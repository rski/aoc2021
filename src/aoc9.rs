use std::fs::read_to_string;

fn aoc(s: &str) {
    let mut nums: Vec<Vec<u32>> = vec![];
    for l in s.lines() {
        dbg!(&l);
        let v: Vec<u32> = l.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
        nums.push(v);
    }
    let l_size = nums[0].len();
    let mut basins: Vec<(usize, usize)> = vec![];
    for (row, vec) in nums.iter().enumerate() {
        for (i, n) in vec.iter().enumerate() {
            let is_min = neighbors(row, i, l_size, &nums).iter().all(|&x| match x {
                None => true,
                Some((row, i)) => *n < nums[row][i],
            });
            if !is_min {
                continue;
            }
            basins.push((row, i));
            println!("line {} i {} number {} is min", i / l_size, i % l_size, n);
        }
    }
    let risk: u32 = basins.iter().map(|(row, i)| nums[*row][*i]).sum::<u32>() + basins.len() as u32;
    dbg!(&basins, risk);
}

fn neighbors(
    row: usize,
    i: usize,
    l_len: usize,
    points: &Vec<Vec<u32>>,
) -> [Option<(usize, usize)>; 4] {
    [
        {
            if row == 0 {
                None
            } else {
                Some((row - 1, i))
            }
        },
        {
            if row + 1 == points.len() {
                None
            } else {
                Some((row + 1, i))
            }
        },
        {
            if i == 0 {
                None
            } else {
                Some((row, i - 1))
            }
        },
        {
            if i + 1 == l_len {
                None
            } else {
                Some((row, i + 1))
            }
        },
    ]
}

fn main() -> std::io::Result<()> {
    let test = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
    aoc(test);
    let buffer = read_to_string(String::from("d9.in"))?;
    aoc(buffer.trim());
    Ok(())
}
