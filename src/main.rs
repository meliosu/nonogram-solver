use std::{io, time::Instant};

fn main() {
    let input = match io::read_to_string(io::stdin()) {
        Ok(string) => string,
        Err(e) => {
            eprintln!("error reading from stdin: {e}");
            return;
        }
    };

    let (rows, cols) = match nonogram::common::parse(input) {
        Some(nonogram) => nonogram,
        None => {
            eprintln!("error parsing nonogram");
            return;
        }
    };

    if let Err(e) = nonogram::common::validate(&rows, &cols) {
        eprintln!("incorrect nonogram: {e}");
        return;
    }

    let start = Instant::now();

    match nonogram::solver_kissat::solve(rows, cols) {
        Some(solution) => {
            let elapsed = start.elapsed();

            println!("SOLUTION:");
            nonogram::common::display(&solution);

            eprintln!("TIME:\n{elapsed:?}");
        }

        None => {
            eprintln!("UNSOLVABLE");
        }
    }
}
