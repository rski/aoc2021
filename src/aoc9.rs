use std::fs::read_to_string;

fn aoc(s: &str) {
    let mut nums: Vec<Vec<u32>> = vec![];
    for l in s.lines() {
        dbg!(&l);
        let v: Vec<u32> = l.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
        nums.push(v);
    }
    let l_size = nums[0].len();
    let mut mins: Vec<u32> = vec![];
    for (row, vec) in nums.iter().enumerate() {
        for (i, n) in vec.iter().enumerate() {
            if (row == 0 || *n < nums[row - 1][i])
                && (row + 1 >= nums.len() || *n < nums[row + 1][i])
                && (i == 0 || *n < vec[i - 1])
                && (i + 1 >= l_size || *n < vec[i + 1])
            {
                mins.push(*n);
                println!("line {} i {} number {} is min", i / l_size, i % l_size, n);
            }
        }
    }
    let risk: u32 = mins.iter().sum::<u32>() + mins.len() as u32;
    dbg!(&mins, risk);
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
