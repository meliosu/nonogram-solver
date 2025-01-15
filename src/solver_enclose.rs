use kissat::{Solver, Var};

use crate::Vec2D;

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
    let num_blocks = cons.len();
    let num_cells = len as usize;

    let after: Vec2D<_> = (0..num_blocks)
        .map(|_| (0..num_cells).map(|_| s.var()).collect())
        .collect();

    let before: Vec2D<_> = (0..num_blocks)
        .map(|_| (0..num_cells).map(|_| s.var()).collect())
        .collect();

    for i in 0..num_cells {
        let mut helpers = Vec::new();

        for j in 0..num_blocks {
            let helper = s.var();

            s.add2(!helper, !after[j][i]);
            s.add2(!helper, !before[j][i]);
            s.add3(helper, after[j][i], before[j][i]);

            helpers.push(helper);
        }

        for &helper in &helpers {
            s.add2(lits[i], !helper);
        }

        helpers.push(!lits[i]);

        s.add(&helpers);
    }

    for i in 1..num_cells {
        for j in 0..num_blocks {
            s.add2(!after[j][i], after[j][i - 1]);
        }
    }

    for i in 0..num_cells - 1 {
        for j in 0..num_blocks {
            s.add2(!before[j][i], before[j][i + 1]);
        }
    }

    for j in 0..num_blocks {
        let len = cons[j] as usize;

        for i in 0..num_cells - len {
            s.add2(after[j][i], before[j][i + len]);
        }
    }

    for j in 0..num_blocks {
        s.add1(after[j][0]);
        s.add1(before[j][num_cells - 1]);
    }
}
