use kissat::{Solver, Var};

use crate::{common::find_solutions, Vec2D};

use ext::SolverExt;

pub fn solve(rows: Vec2D<u32>, cols: Vec2D<u32>) -> Option<Vec2D<bool>> {
    let mut s = Solver::new();

    let width = cols.len();
    let height = rows.len();

    let cells: Vec2D<_> = (0..height)
        .map(|_| (0..width).map(|_| s.var()).collect())
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

    let solution = s.sat()?;

    let answer = cells
        .iter()
        .map(|row| row.iter().map(|&c| solution.get(c).unwrap()).collect())
        .collect();

    Some(answer)
}

fn add_condition(s: &mut Solver, lits: &[Var], cons: &[u32], len: u32) {
    let lits: Vec<_> = find_solutions(cons, len)
        .into_iter()
        .map(|solution| {
            s.and_literal(
                &std::iter::zip(solution, lits)
                    .map(|(c, &lit)| if c { lit } else { !lit })
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    s.add(&lits);
}

mod ext {
    use std::iter::once;

    use kissat::{Solver, Var};

    pub(super) trait SolverExt {
        fn and_literal(&mut self, lits: &[Var]) -> Var;
    }

    impl SolverExt for Solver {
        fn and_literal(&mut self, lits: &[Var]) -> Var {
            let res = self.var();

            for &lit in lits {
                self.add2(!res, lit);
            }

            self.add(
                &lits
                    .into_iter()
                    .map(|&lit| !lit)
                    .chain(once(res))
                    .collect::<Vec<_>>(),
            );

            res
        }
    }
}
