use std::{
    collections::{BTreeSet, HashMap},
    fs::read_to_string,
    ops::Sub,
};

fn aoc(input: &str) {
    let width_to_number = HashMap::from([(2, 1), (4, 4), (3, 7), (7, 8)]);
    let mut vals = 0;
    for l in input.lines() {
        let mut sets = HashMap::<u64, BTreeSet<char>>::new();
        let mut split = l.split("|");
        let all_patterns = split.next().unwrap().trim().split(" ");
        for pattern in all_patterns.clone() {
            let w = &pattern.chars().count();
            match width_to_number.get(w) {
                None => {}
                Some(x) => {
                    let mut s = BTreeSet::new();
                    for c in pattern.chars() {
                        s.insert(c);
                    }
                    sets.insert(*x, s);
                }
            }
        }
        for pattern in all_patterns.clone() {
            if pattern.chars().count() != 6 {
                continue;
            }
            let four = sets.get(&4).unwrap();
            let mut s = BTreeSet::new();
            for c in pattern.chars() {
                s.insert(c);
            }
            if four.is_subset(&s) {
                sets.insert(9, s);
            } else {
                let one = sets.get(&1).unwrap();
                if one.is_subset(&s) {
                    sets.insert(0, s);
                } else {
                    sets.insert(6, s);
                }
            }
        }
        for pattern in all_patterns.clone() {
            if pattern.chars().count() != 5 {
                continue;
            }

            let mut s = BTreeSet::new();
            for c in pattern.chars() {
                s.insert(c);
            }
            let one = sets.get(&1).unwrap();
            if one.is_subset(&s) {
                sets.insert(3, s);
            } else {
                let six = sets.get(&6).unwrap();
                if s.sub(&six).len() == 0 {
                    sets.insert(5, s);
                } else {
                    sets.insert(2, s);
                }
            }
        }
        let mut rev_sets = HashMap::<BTreeSet<char>, u64>::new();
        for (k, v) in sets {
            rev_sets.insert(v, k);
        }
        let output = split.next().unwrap().trim().split(" ");
        let mut day_value: u64 = 0;
        for (i, pattern) in output.enumerate() {
            let mut s = BTreeSet::new();
            for c in pattern.chars() {
                s.insert(c);
            }
            let v = rev_sets.get(&s).unwrap();
            day_value += 10_u64.pow(4 - 1 - i as u32) * v;
            dbg!(pattern, v);
        }
        vals += day_value
    }

    println!("total:{:?}", vals);
}
fn main() -> std::io::Result<()> {
    let test = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
    aoc(test);
    let buffer = read_to_string(String::from("d8.in"))?;
    aoc(buffer.trim());
    Ok(())
}
