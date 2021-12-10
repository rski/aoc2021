use std::fs::read_to_string;

enum State {
    Corrupt(u64),
    Incomplete(u64),
}

fn aoc(s: &str) -> (u64, u64) {
    let mut score = 0;
    let mut completion_scores = Vec::<u64>::new();
    for l in s.lines() {
        match calc(l) {
            State::Corrupt(n) => score += n,
            State::Incomplete(n) => completion_scores.push(n),
        }
    }
    completion_scores.sort_unstable();

    (score, completion_scores[(completion_scores.len() / 2)])
}

fn calc(l: &str) -> State {
    let mut chars = Vec::<char>::new();
    for c in l.chars() {
        match c {
            '{' | '[' | '<' | '(' => chars.push(c),
            '}' => {
                if chars.pop().unwrap() != '{' {
                    return State::Corrupt(1197);
                }
            }
            ']' => {
                if chars.pop().unwrap() != '[' {
                    return State::Corrupt(57);
                }
            }
            '>' => {
                if chars.pop().unwrap() != '<' {
                    return State::Corrupt(25137);
                }
            }
            ')' => {
                if chars.pop().unwrap() != '(' {
                    return State::Corrupt(3);
                }
            }
            _ => panic!("unexpected char {}", c),
        }
    }
    let mut score = 0;
    for c in chars.iter().rev() {
        score = (score * 5)
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("unexpected {}", c),
            }
    }
    State::Incomplete(score)
}
fn main() -> std::io::Result<()> {
    let test = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
    let (score, ac) = aoc(test);
    assert_eq!(score, 26397);
    assert_eq!(ac, 288957);
    let buffer = read_to_string(String::from("d10.in"))?;
    let (score, ac) = aoc(buffer.trim());
    assert_eq!(score, 315693);
    assert_eq!(ac, 1870887234);
    Ok(())
}
