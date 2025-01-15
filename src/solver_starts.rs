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

fn add_condition(s: &mut Solver, lits: &[Var], cons: &[u32], _len: u32) {
    let num_blocks = cons.len();
    let num_cells = lits.len();

    let starts = (0..num_blocks)
        .map(|_| (0..num_cells).map(|_| s.var()).collect())
        .collect::<Vec2D<_>>();

    // First
    //
    // If Sj starts on cell i, then cells [i, i + len) are colored
    //
    // Sj_i -> (C_i & C_i+1 & .. & C_i+len-1)
    for j in 0..num_blocks {
        let len = cons[j] as usize;

        for i in 0..num_cells - len + 1 {
            for k in i..i + len {
                s.add2(!starts[j][i], lits[k]);
            }
        }

        for i in num_cells - len + 1..num_cells {
            s.add1(!starts[j][i]);
        }
    }

    // Second
    //
    // If cell is colored, then it must be due to some block
    //
    // C_i -> (S v S v ... v S)
    for i in 0..num_cells {
        let mut clause = Vec::new();

        clause.push(!lits[i]);

        for j in 0..num_blocks {
            let len = cons[j] as usize;

            for k in (0..=i).rev().take(len) {
                clause.push(starts[j][k]);
            }
        }

        s.add(&clause);
    }

    // Third
    //
    // There can be only one start per block
    //
    // one_of(Sj_0, Sj_1, ..., Sj_n)
    for j in 0..num_blocks {
        s.add(&starts[j]);

        for i in 0..num_cells {
            for k in i + 1..num_cells {
                s.add2(!starts[j][i], !starts[j][k]);
            }
        }
    }

    // Fourth
    //
    // Blocks must be ordered
    for j in 0..num_blocks - 1 {
        let curr = &starts[j];
        let next = &starts[j + 1];

        let len = cons[j] as usize;

        for i in 0..num_cells - len - 1 {
            let mut clause = Vec::new();

            clause.push(!curr[i]);

            for k in i + len + 1..num_cells {
                clause.push(next[k]);
            }

            s.add(&clause);
        }
    }
}
