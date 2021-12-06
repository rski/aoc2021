use std::fs::read_to_string;

struct Subnautica {
    d: i32,
    fwd: i32,
    aim: i32,
    aim_depth: i32,
}

fn main() -> std::io::Result<()> {
    let buffer = read_to_string(String::from("d2.in"))?;
    let directions = buffer.lines().map(|l| {
        let mut it = l.split_ascii_whitespace();
        let direction = it.next().unwrap();
        let v = it.next().unwrap().parse::<i32>().unwrap();
        (direction, v)
    });

    let mut s = Subnautica {
        d: 0,
        fwd: 0,
        aim: 0,
        aim_depth: 0,
    };
    for d in directions {
        match d {
            ("forward", x) => {
                s.fwd += x;
                s.aim_depth += s.aim * x;
            }
            ("down", x) => {
                s.d += x;
                s.aim += x;
            }
            ("up", x) => {
                s.d -= x;
                s.aim -= x;
            }
            (d, _) => panic!("unexpected direction {}", d),
        };
    }
    println!(
        "aoc2_p1: {}, aim_depth:{}",
        s.d * s.fwd,
        s.fwd * s.aim_depth
    );

    Ok(())
}
