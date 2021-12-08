use std::{collections::HashMap, fs::read_to_string};

fn aoc(input: &str) {
    let width_to_number = HashMap::from([(2, 1), (3, 7), (4, 4), (7, 8)]);
    let mut freq = HashMap::<u64, u64>::new();
    for l in input.lines() {
        let split = l.split("|");
        let output = split.skip(1).next().unwrap();
        for pattern in output.split(" ") {
            if pattern == "" {
                continue;
            }
            match width_to_number.get(&pattern.chars().count()) {
                None => {}
                Some(x) => {
                    let prev = freq.get(&x).unwrap_or(&0) + 1;
                    freq.insert(*x, prev);
                }
            }
        }
    }
    println!("total:{}", freq.values().sum::<u64>());
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
