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

    let solution = if let Some(solver) = std::env::args().nth(1) {
        if solver == "minisat" {
            nonogram::solver_minisat::solve(rows, cols)
        } else if solver == "kissat" {
            nonogram::solver_kissat::solve(rows, cols)
        } else if solver == "automaton" {
            nonogram::solver_automaton::solve(rows, cols)
        } else if solver == "automaton-minisat" {
            nonogram::solver_automaton_minisat::solve(rows, cols)
        } else if solver == "enclose" {
            nonogram::solver_enclose::solve(rows, cols)
        } else if solver == "starts" {
            nonogram::solver_starts::solve(rows, cols)
        } else {
            eprintln!("wrong solver name");
            return;
        }
    } else {
        nonogram::solver_kissat::solve(rows, cols)
    };

    let elapsed = start.elapsed();

    if let Some(solution) = solution {
        println!("SOLUTION:");
        nonogram::common::display(&solution);

        eprintln!("TIME:\n{elapsed:?}");
    } else {
        eprintln!("UNSOLVABLE");
    }
}
