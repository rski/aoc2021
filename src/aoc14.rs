use std::{
    collections::{BTreeSet, HashMap},
    fs::read_to_string,
    slice::SliceIndex,
};

fn count_pop(count: &HashMap<char, u64>) -> u64 {
    let mut min_v = u64::MAX;
    let mut max_v = 0;
    for (_, &v) in count {
        if v > max_v {
            max_v = v;
        }
        if v < min_v {
            min_v = v
        }
    }
    dbg!(&count);
    max_v - min_v
}
fn mutate(input: &str, steps: u64) -> u64 {
    let mut start = String::new();
    let mut rules = HashMap::<(char, char), char>::new();

    let mut parsing_rules = false;
    for l in input.lines() {
        if l.is_empty() {
            parsing_rules = true;
            continue;
        }
        if !parsing_rules {
            start = l.to_owned();
        } else {
            let mut split = l.split(' ');
            let mut input = split.next().unwrap().chars();
            let (a, b) = (input.next().unwrap(), input.next().unwrap());
            split.next();
            let res = split.next().unwrap();
            rules.insert((a, b), res.chars().next().unwrap());
        }
    }
    let mut poly = HashMap::<(char, char), u64>::new();
    let mut count = HashMap::<char, u64>::new();
    for p in start.chars().zip(start.clone().chars().skip(1)) {
        let e = poly.entry(p).or_insert(0);
        *e += 1;
    }
    for k in start.chars() {
        let e = count.entry(k).or_insert(0);
        *e += 1;
    }

    for _ in 0..steps {
        let mut new_poly = HashMap::<(char, char), u64>::new();
        for (pair, v) in poly.iter() {
            match rules.get(pair) {
                None => {
                    new_poly.insert(*pair, *v);
                }
                Some(new) => {
                    let e = new_poly.entry((pair.0, *new)).or_insert(0);
                    *e += v;
                    let e = new_poly.entry((*new, pair.1)).or_insert(0);
                    *e += v;
                    let e = count.entry(*new).or_insert(0);
                    *e += v;
                }
            }
        }
        poly = new_poly;
    }
    dbg!(&rules);
    count_pop(&count)
}
fn main() -> std::io::Result<()> {
    let test = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    dbg!(mutate(test, 1));
    // let count = count_pop(&mutation);
    c("NCNBCHB");
    dbg!(mutate(test, 2));
    // let count = count_pop(&mutation);
    c("NBCCNBBBCBHCB");
    dbg!(mutate(test, 3));
    c("NBBBCNCCNBBNBNBBCHBHHBCHB");
    dbg!(mutate(test, 4));
    c("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");

    let pop = mutate(test, 10);
    assert_eq!(pop, 1588);

    let input = read_to_string("d14.in").unwrap();
    let pop = mutate(&input, 10);
    assert_eq!(pop, 2937);
    let pop = mutate(&input, 40);
    assert_eq!(pop, 3390034818249);
    Ok(())
}

fn c(i: &str) {
    let mut example = HashMap::<char, u64>::new();
    for c in i.chars() {
        let v = example.get(&c).unwrap_or(&0) + 1;
        example.insert(c, v);
    }
    dbg!(&example);
}
