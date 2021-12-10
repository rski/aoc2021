use std::cmp::{max, min};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Line(Point, Point);

impl Line {
    fn new(s: Point, e: Point) -> Line {
        if e > s {
            Line(s, e)
        } else {
            Line(e, s)
        }
    }

    fn fill_grid(&self, grid: &mut HashMap<String, u64>) {
        if self.0 .0 == self.1 .0 {
            let fmt = |y: u64| format!("{},{}", self.0 .0, y);
            for y in min(self.0 .1, self.1 .1)..=max(self.0 .1, self.1 .1) {
                let p = fmt(y);
                let prev_value = grid.get(&p).unwrap_or(&0) + 1;
                grid.insert(p, prev_value);
            }
        } else if self.0 .1 == self.1 .1 {
            let fmt = |x: u64| format!("{},{}", x, self.0 .1);
            for x in min(self.0 .0, self.1 .0)..=max(self.0 .0, self.1 .0) {
                let p = fmt(x);
                let prev_value = grid.get(&p).unwrap_or(&0) + 1;
                grid.insert(p, prev_value);
            }
        } else if self.0 .0 == self.1 .1 {
            let mut x = self.0 .0;
            let mut y = self.0 .1;
            loop {
                let p = format!("{},{}", x, y);
                let prev_value = grid.get(&p).unwrap_or(&0) + 1;
                grid.insert(p, prev_value);
                if x == self.1 .0 {
                    break;
                }
                if self.0 .0 > self.0 .1 {
                    x -= 1;
                    y += 1;
                } else {
                    x += 1;
                    y -= 1;
                }
            }
        } else {
            let slope: i32 =
                (self.0 .0 as i32 - self.1 .0 as i32) / (self.0 .1 as i32 - self.1 .1 as i32);
            if slope.abs() != 1 {
                println!("nope!{:?}->{}", self, slope);
                return;
            }
            println!("45 degree: {:?}, {}", self, slope);
            let mut x = self.0 .0;
            let mut y = self.0 .1;
            let inc_x = self.0 .0 < self.1 .0;
            let inc_y = self.0 .1 < self.1 .1;
            loop {
                let p = format!("{},{}", x, y);
                let prev_value = grid.get(&p).unwrap_or(&0) + 1;
                grid.insert(p, prev_value);
                if x == self.1 .0 {
                    break;
                }
                if inc_x {
                    x += 1;
                } else {
                    x -= 1;
                }
                if inc_y {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        };
    }
}

#[derive(Debug, PartialEq)]
struct Point(u64, u64);
impl Point {
    fn distance_from_origin_sq(&self) -> u64 {
        self.0.pow(2) * self.1.pow(2)
    }
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(',').collect();
        let a = s[0].parse::<u64>()?;
        let b = s[1].parse::<u64>()?;
        Ok(Point(a, b))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.distance_from_origin_sq()
                .cmp(&other.distance_from_origin_sq()),
        )
    }
}

fn parse_lines(lines: String) -> Vec<Line> {
    let mut points: Vec<Line> = vec![];
    for l in lines.lines() {
        let mut l = l.split_ascii_whitespace();
        let s = Point::from_str(l.next().unwrap()).unwrap();
        let e = Point::from_str(l.last().unwrap()).unwrap();
        points.push(Line::new(s, e));
    }
    points
}

fn main() -> std::io::Result<()> {
    let buffer = read_to_string(String::from("d5.in"))?;
    let lines = parse_lines(buffer);
    let mut grid = HashMap::<String, u64>::new();
    for l in lines.into_iter() {
        l.fill_grid(&mut grid);
    }
    let nums = grid.values().filter(|v| **v >= 2).count();
    dbg!(nums);
    Ok(())
}
