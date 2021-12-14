use std::{collections::HashMap, fs::read_to_string};

fn count_pop(mutation: &str) -> u32 {
    let mut pop = HashMap::<char, u32>::new();
    for c in mutation.chars() {
        let v = pop.get(&c).unwrap_or(&0) + 1;
        pop.insert(c, v);
    }
    let mut min_v = u32::MAX;
    let mut max_v = 0;
    for (_, v) in pop {
        if v > max_v {
            max_v = v;
        }
        if v < min_v {
            min_v = v
        }
    }
    println!("{}, {}", max_v, min_v);

    max_v - min_v
}
fn mutate(input: &str, steps: u32) -> String {
    let mut start = String::new();
    let mut rules = HashMap::<&str, char>::new();
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
            let input = split.next().unwrap();
            split.next();
            let res = split.next().unwrap();
            rules.insert(input, res.chars().next().unwrap());
        }
    }
    // dbg!(&rules);
    for _ in 0..steps {
        let mut mutations = Vec::<char>::new();
        for i in 0..start.len() - 1 {
            let k = &start[i..i + 2];
            match rules.get(k) {
                None => panic!("error finding key {}", k),
                Some(v) => mutations.push(*v),
            };
        }
        // println!("{:?}", mutations);
        for (i, m) in mutations.iter().enumerate() {
            start.insert(i * 2 + 1, *m);
        }
    }
    start
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
    let mutation = mutate(test, 4);
    assert_eq!(
        mutation,
        String::from("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")
    );
    let mutation = mutate(test, 10);
    let count = count_pop(&mutation);
    assert_eq!(count, 1588);

    let input = read_to_string("d14.in").unwrap();
    let mutation = mutate(&input, 10);
    let count = count_pop(&mutation);
    assert_eq!(count, 2937);

    Ok(())
}
