use std::{collections::BTreeSet, fs::read_to_string};

fn aoc(s: &str) -> (usize, usize) {
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
            let is_min = neighbors(row, i, l_size, &nums)
                .iter()
                .all(|&x| *n < nums[x.0][x.1]);
            if !is_min {
                continue;
            }
            basins.push((row, i));
            println!("line {} i {} number {} is min", i / l_size, i % l_size, n);
        }
    }
    let risk: u32 = basins.iter().map(|(row, i)| nums[*row][*i]).sum::<u32>() + basins.len() as u32;
    let mut basin_sizes = basins
        .iter()
        .map(|&s| calc_basin(s, l_size, &nums))
        .collect::<Vec<usize>>();

    basin_sizes.sort();
    (
        risk as usize,
        basin_sizes.iter().rev().take(3).product::<usize>(),
    )
}

fn calc_basin(seed: (usize, usize), l_size: usize, points: &Vec<Vec<u32>>) -> usize {
    let mut basin = BTreeSet::<(usize, usize)>::new();
    basin.insert(seed);
    let mut prev_set = basin.clone();
    loop {
        if prev_set.len() == 0 {
            break;
        }
        let mut new_points = BTreeSet::<(usize, usize)>::new();
        for p in &prev_set {
            for n in &neighbors(p.0, p.1, l_size, points) {
                if points[n.0][n.1] != 9 && !basin.contains(n) {
                    basin.insert(*n);
                    new_points.insert(*n);
                }
            }
        }
        prev_set = new_points;
    }
    basin.len()
}

fn neighbors(row: usize, i: usize, l_len: usize, points: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut n = Vec::<(usize, usize)>::new();
    if row != 0 {
        n.push((row - 1, i));
    }

    if row + 1 != points.len() {
        n.push((row + 1, i));
    }
    if i != 0 {
        n.push((row, i - 1))
    }
    if i + 1 != l_len {
        n.push((row, i + 1));
    }
    n
}

fn main() -> std::io::Result<()> {
    let test = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
    let (risk, basins) = aoc(test);
    assert_eq!(risk, 15);
    assert_eq!(basins, 1134);
    let buffer = read_to_string(String::from("d9.in"))?;
    let (risk, basins) = aoc(buffer.trim());
    assert_eq!(risk, 506);
    assert_eq!(basins, 931200);
    Ok(())
}
