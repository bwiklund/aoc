enum Op {
    Add,
    Multiply,
}

impl Op {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Self::Add),
            "*" => Some(Self::Multiply),
            _ => None,
        }
    }
}

pub fn solve(part: u32) -> u64 {
    let txt = std::fs::read_to_string("./src/day6_input.txt").unwrap();
    let lines = txt.lines().collect::<Vec<_>>();
    let columns: Vec<Vec<u64>> = match part {
        0 => {
            let rows = lines[0..lines.len() - 1]
                .iter()
                .map(|l| {
                    l.split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<_>>();

            // transpose
            (0..rows[0].len())
                .map(|col_idx| {
                    (0..rows.len())
                        .map(|row_idx| rows[row_idx][col_idx])
                        .collect()
                })
                .collect()
        }

        1 => todo!("ASDF"),

        _ => panic!(),
    };

    let ops = lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| Op::parse(s).unwrap());

    ops.enumerate()
        .map(|(idx, op)| {
            let operands = columns[idx].iter();
            match op {
                Op::Add => operands.sum::<u64>(),
                Op::Multiply => operands.product(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6() {
        assert_eq!(solve(0), 6891729672676);
    }
}
