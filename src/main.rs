mod naive;
mod optimized;

use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

trait AddNumber {
    fn add_number(&mut self, number: u128);
}

fn read_lines(path: String) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

fn run<State: AddNumber + Default>(lines: Lines<BufReader<File>>) {
    let mut state = State::default();
    for line in lines.flatten() {
        state.add_number(line.parse().expect("A valid number per line should be set"));
    }
}

fn main() -> io::Result<()> {
    let mut args = env::args();
    match (args.nth(1), args.next()) {
        (Some(algorithm), Some(path)) => match algorithm.as_str() {
            "naive" => run::<naive::State>(read_lines(path)?),
            "optimized" => run::<optimized::State>(read_lines(path)?),
            other => panic!("Algorithm not found: {}", other),
        },
        (Some(_), None) => panic!("Missing file path argument"),
        (None, _) => panic!("Missing algorithm argument"),
    }
    Ok(())
}
