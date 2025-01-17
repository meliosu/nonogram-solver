use crate::Vec2D;

pub(crate) fn find_solutions(cons: &[u32], len: u32) -> Vec2D<bool> {
    let curr = match cons.first() {
        Some(&n) => n,
        None => {
            return vec![vec![false; len as usize]];
        }
    };

    if curr > len {
        return vec![];
    }

    let mut results = Vec::new();

    for start in 0..=(len - curr) {
        let mut part = Vec::new();

        for _ in 0..start {
            part.push(false);
        }

        for _ in 0..curr {
            part.push(true);
        }

        if len - (start + curr) == 0 && cons.len() == 1 {
            results.push(part);
        } else if len - (start + curr) > 0 {
            part.push(false);

            for subresult in find_solutions(&cons[1..], len - (start + curr + 1)) {
                let mut result = part.clone();
                result.extend(subresult);
                results.push(result);
            }
        }
    }

    results
}

pub fn parse(input: String) -> Option<(Vec2D<u32>, Vec2D<u32>)> {
    let mut lines = input.lines();

    let (num_rows, num_cols) = lines
        .next()
        .and_then(|line| line.split_once(' '))
        .and_then(|(rows, cols)| Option::zip(rows.parse().ok(), cols.parse().ok()))?;

    let mut collect = |num| {
        (0..num).try_fold(Vec::new(), |mut acc, _| {
            let line = lines.next()?;
            let nums = line.split_whitespace().try_fold(Vec::new(), |mut acc, n| {
                let n = n.parse().ok()?;
                acc.push(n);
                Some(acc)
            })?;

            acc.push(nums);
            Some(acc)
        })
    };

    Some((collect(num_rows)?, collect(num_cols)?))
}

pub fn display(solution: &Vec2D<bool>) {
    print!("╔");

    for _ in 0..solution[0].len() {
        print!("═");
    }

    print!("╗\n");

    for row in solution {
        print!("║");

        for &cell in row {
            print!("{}", if cell { '#' } else { '.' });
        }

        print!("║\n");
    }

    print!("╚");

    for _ in 0..solution[0].len() {
        print!("═");
    }

    print!("╝");
    print!("\n");
}

pub fn validate(rows: &Vec2D<u32>, cols: &Vec2D<u32>) -> Result<(), String> {
    let validate = |constraints: &Vec2D<u32>, len: usize| {
        for (i, cs) in constraints.iter().enumerate() {
            if cs.iter().sum::<u32>() + cs.len() as u32 - 1 > len as u32 {
                return Err(format!("in position {}", i + 1));
            }

            if cs.contains(&0) {
                return Err(format!("in position {}", i + 1));
            }
        }

        Ok(())
    };

    let height = rows.len();
    let width = cols.len();

    validate(rows, width).map_err(|e| format!("invalid row constraint: {e}"))?;
    validate(cols, height).map_err(|e| format!("invalid column constraint: {e}"))?;

    Ok(())
}
