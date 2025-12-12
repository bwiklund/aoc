struct Shape {
    w: usize,
    h: usize,
    cells: Vec<bool>,
}

#[derive(Debug)]
enum ParsedLine {
    ShapeId(u64),
    ShapeRow(Vec<bool>),
    RegionRow(u64, u64, Vec<u64>),
}

fn expect_number(chars: &Vec<char>, i: &mut usize) -> u64 {
    let start = *i;
    while chars[*i].is_digit(10) {
        *i += 1;
    }
    chars[start..*i].iter().collect::<String>().parse().unwrap()
}

pub fn solve(part: u32) -> u64 {
    // let shapes = vec![];
    // let regions = vec![];

    let txt = std::fs::read_to_string("./src/day12_input.txt").unwrap();

    let lines = txt.lines().filter_map(|l| {
        let chars = l.chars().collect::<Vec<_>>();
        let mut i = 0;
        return if chars.is_empty() {
            None
        } else if chars[i].is_digit(10) {
            let n1 = expect_number(&chars, &mut i);

            if chars[i] == 'x' {
                i += 1;
                let n2 = expect_number(&chars, &mut i);
                Some(ParsedLine::RegionRow(n1, n2, vec![]))
            } else if chars[i] == ':' {
                Some(ParsedLine::ShapeId(n1))
            } else {
                dbg!(chars);
                panic!()
            }
        } else if matches!(chars[i], '.' | '#') {
            // we're in a shape row
            Some(ParsedLine::ShapeRow(
                chars
                    .iter()
                    .map(|ch| match ch {
                        '#' => true,
                        '.' => false,
                        _ => panic!(),
                    })
                    .collect(),
            ))
        } else {
            // panic!();
            None
        };
    });

    dbg!(lines.take(10).collect::<Vec<_>>());

    match part {
        0 => 0,

        1 => 0,

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
