use core::fmt;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::iter::Iterator;
use std::ops::Rem;

struct BingoBoard {
    size: usize,
    numbers: Vec<Option<u64>>,
    moves_to_win: usize,
    winning_play: Option<u64>,
}

impl Debug for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, n) in self.numbers.iter().enumerate() {
            if i % self.size == 0 {
                write!(f, "\n")?;
            }
            match n {
                Some(n) => write!(f, "\t{} ", n)?,
                None => write!(f, "\tXX")?,
            };
        }
        write!(
            f,
            "\nwon:{:?}, moves:{}, score:{}\n",
            self.winning_play,
            self.moves_to_win,
            self.score(),
        )?;
        Ok(())
    }
}

impl BingoBoard {
    fn new(lines: &mut std::str::Lines) -> BingoBoard {
        let mut b = BingoBoard {
            size: 0,
            numbers: vec![],
            moves_to_win: 0,
            winning_play: None,
        };
        for l in lines {
            if l == "" {
                break;
            }
            let vals: Vec<Option<u64>> = l
                .split_ascii_whitespace()
                .filter(|&x| !x.is_empty())
                .map(|x| Some(x.parse::<u64>().unwrap()))
                .collect();
            b.size = vals.len();
            b.numbers.extend_from_slice(vals.as_slice());
        }
        b
    }
    fn play(&mut self, n: u64) {
        if self.winning_play.is_some() {
            return;
        }
        self.moves_to_win += 1;
        let mut rows: HashMap<usize, usize> = HashMap::new();
        let mut columns: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.numbers.len() {
            if self.numbers[i] == Some(n) {
                self.numbers[i] = None;
            }
        }
        for (i, v) in self.numbers.iter().enumerate() {
            if *v == None {
                let column = i.rem(self.size);
                let new = columns.get(&column).unwrap_or(&self.size) - 1;
                if new == 0 {
                    println!("column {} is empty, won!", column);
                    self.winning_play = Some(n);
                    return;
                }
                columns.insert(column, new);

                let row = i / self.size;
                let new = rows.get(&row).unwrap_or(&self.size) - 1;
                if new == 0 {
                    println!("row {} is empty, won!", row);
                    self.winning_play = Some(n);
                    return;
                }
                rows.insert(row, new);
            }
        }
    }
    fn sum(&self) -> u64 {
        self.numbers.iter().fold(0, |acc, item| match item {
            Some(n) => acc + n,
            None => acc,
        })
    }
    fn score(&self) -> u64 {
        match self.winning_play {
            None => 0,
            Some(n) => n * self.sum(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let (plays, mut boards) = setup()?;

    play_bingo(&plays, &mut boards);

    for b in &boards {
        dbg!(b);
    }

    let (
        mut fewest_plays_to_win,
        mut most_plays_to_win,
        mut first_winner_score,
        mut last_winner_score,
    ) = (plays.len(), 0, 0, 0);
    for board in boards.iter() {
        let p = board.moves_to_win;
        if p > most_plays_to_win {
            most_plays_to_win = p;
            last_winner_score = board.score();
        } else if p < fewest_plays_to_win {
            fewest_plays_to_win = p;
            first_winner_score = board.score();
        }
    }

    dbg!(first_winner_score, last_winner_score);
    Ok(())
}

fn setup() -> Result<(Vec<u64>, Vec<Box<BingoBoard>>), std::io::Error> {
    let buffer = read_to_string(String::from("d4.in"))?;
    let mut lines = buffer.lines().into_iter();
    let plays: Vec<u64> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    dbg!(&plays);
    lines.next();
    let mut boards: Vec<Box<BingoBoard>> = vec![];
    loop {
        let b = BingoBoard::new(&mut lines);
        dbg!(&b);
        if b.size == 0 {
            break;
        }
        boards.push(Box::new(b));
    }
    Ok((plays, boards))
}

fn play_bingo(plays: &Vec<u64>, boards: &mut Vec<Box<BingoBoard>>) {
    for p in plays {
        println!("playing {}", p);
        for b in boards.iter_mut() {
            b.play(*p);
        }
    }
}
