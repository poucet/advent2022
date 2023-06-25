use common::read_input;
use day4::{Board, Input};

fn main() {
    let contents = read_input() + "\n\n";
    let input = Input::parse(&contents).unwrap().1;
    if false {
        println!("Score: {}", process(input));
    } else {
        println!("Score: {}", process2(input));
    }
}

fn process(mut input: Input) -> u64 {
    for n in input.draws {
        for b in input.boards.iter_mut() {
            b.mark(n);
            if b.is_complete() {
                return n * b.score()
            }
        }
    }
    0
}

fn process2(mut input: Input) -> u64 {
    for n in input.draws {        
        input.boards.retain(|b| !b.is_complete());
        for b in input.boards.iter_mut() {
            b.mark(n);
        }
        if input.boards.len() == 1 && input.boards[0].is_complete() {
            return dbg!(n) * input.boards[0].score();
        }   
    }
    0
}