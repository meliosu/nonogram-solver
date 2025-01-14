use minisat::{Bool, Solver};

use crate::common::find_solutions;
use crate::Vec2D;

pub fn solve(rows: Vec2D<u32>, cols: Vec2D<u32>) -> Option<Vec2D<bool>> {
    let mut s = Solver::new();

    let width = cols.len();
    let height = rows.len();

    let cells: Vec2D<_> = (0..height)
        .map(|_| (0..width).map(|_| s.new_lit()).collect())
        .collect();

    let rows_lits: Vec2D<_> = (0..height)
        .map(|row| (0..width).map(|col| cells[row][col]).collect())
        .collect();

    let cols_lits: Vec2D<_> = (0..width)
        .map(|col| (0..height).map(|row| cells[row][col]).collect())
        .collect();

    for (row, lits) in std::iter::zip(&rows, &rows_lits) {
        add_condition(&mut s, lits, row, width as u32);
    }

    for (col, lits) in std::iter::zip(&cols, &cols_lits) {
        add_condition(&mut s, lits, col, height as u32);
    }

    eprintln!("Solver:");
    eprintln!("- {} vars", s.num_vars());
    eprintln!("- {} clauses", s.num_clauses());

    let model = s.solve().ok()?;

    let answer = cells
        .iter()
        .map(|row| row.iter().map(|c| model.value(c)).collect())
        .collect();

    Some(answer)
}

fn add_condition(s: &mut Solver, lits: &[Bool], cons: &[u32], len: u32) {
    let lits: Vec<_> = find_solutions(cons, len)
        .into_iter()
        .map(|solution| {
            s.and_literal(
                std::iter::zip(solution, lits).map(|(c, &lit)| if c { lit } else { !lit }),
            )
        })
        .collect();

    s.add_clause(lits);
}
