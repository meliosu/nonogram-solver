use minisat::{Bool, Solver};

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

    let solution = s.solve().ok()?;

    let answer = cells
        .iter()
        .map(|row| row.iter().map(|&c| solution.value(&c)).collect())
        .collect();

    Some(answer)
}

fn add_condition(s: &mut Solver, lits: &[Bool], cons: &[u32], len: u32) {
    let len = len as usize;

    let num_states = cons.iter().map(|&n| n as usize).sum::<usize>() + cons.len() + 1;

    let mut transitions = Vec::new();

    for (i, c) in cons.iter().enumerate() {
        if i == 0 {
            transitions.push((Some(true), Some(false)));
        }

        for _ in 0..(c - 1) {
            transitions.push((Some(true), None));
        }

        transitions.push((Some(false), None));

        if i == cons.len() - 1 {
            transitions.push((None, Some(false)));
        } else {
            transitions.push((Some(true), Some(false)));
        }
    }

    let states: Vec2D<_> = (0..num_states)
        .map(|_| (0..len).map(|_| s.new_lit()).collect())
        .collect();

    for i in 1..len {
        let cell = lits[i];

        for state in 0..num_states {
            let curr = states[state][i - 1];

            let transition = transitions[state];

            let is = |v: Bool, b: bool| if b { v } else { !v };

            match transition {
                (Some(next), Some(same)) => {
                    let looped = states[state][i];
                    let success = states[state + 1][i];

                    s.add_clause([!curr, !is(cell, next), success]);
                    s.add_clause([!curr, !is(cell, same), looped]);
                }

                (Some(next), None) => {
                    let success = states[state + 1][i];

                    s.add_clause([!curr, is(cell, next)]);
                    s.add_clause([!curr, success]);
                }

                (None, Some(same)) => {
                    let looped = states[state][i];

                    s.add_clause([!curr, is(cell, same)]);
                    s.add_clause([!curr, looped]);
                }

                (None, None) => unreachable!(),
            }
        }
    }

    // end condition
    // only end state and last cell of last block are valid states
    s.add_clause([
        states[num_states - 2][len - 1],
        states[num_states - 1][len - 1],
    ]);

    // start condition
    // if first cell is picked, go to 2nd state
    // if it is not, remain in the start state
    s.add_clause([lits[0], states[0][0]]);
    s.add_clause([!lits[0], states[1][0]]);

    // at each timestep only one state is valid
    for i in 0..len {
        for j in 0..num_states {
            for k in 0..num_states {
                if j == k {
                    continue;
                }

                s.add_clause([!states[j][i], !states[k][i]]);
            }
        }
    }
}
