use std::fmt::{Display, Formatter, Write};
use crate::Piece::X;

#[derive(Debug, Clone)]
enum Piece{
    W,
    B,
    X
}

#[derive(Debug)]
struct Board{
    board: Vec<Vec<Piece>>
}

impl Board {
    fn new()->Board{
        Board{
            board: vec![
                vec![X; 8];8
            ],
        }
    }
}

impl Display for Board{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for p in &self.board{
            for pp in p{
                write!(f, "{:?} ", pp).expect("TODO;")
            }
            writeln!(f).expect("TODO;")
        }
        Ok(())
    }
}

fn main() {
    let b = Board::new();
    println!("{}", b)
}
