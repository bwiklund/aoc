#[derive(Debug)]
struct Grid {
    w: usize,
    h: usize,
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn new(rows: Vec<Vec<char>>) -> Option<Self> {
        if let Some(w) = rows.first().map(|row| row.len())
            && rows.iter().all(|row| row.len() == w)
        {
            Some(Self {
                h: rows.len(),
                w: w,
                rows,
            })
        } else {
            None
        }
    }

    fn get(&self, x: i64, y: i64) -> Option<&char> {
        self.rows
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }

    fn has_str(&self, search: &str, x: i64, y: i64, dx: i64, dy: i64) -> bool {
        if dx == 0 && dy == 0 {
            return false;
        }

        search.chars().enumerate().all(|(i, search_ch)| {
            self.get(x + dx * i as i64, y + dy * i as i64)
                .map(|&ch| ch == search_ch)
                .unwrap_or(false)
        })
    }
}

pub fn solve(part: u32) -> i64 {
    let grid = Grid::new(
        std::fs::read_to_string("./src/day4_input.txt")
            .unwrap()
            .lines()
            .map(|l| l.chars().collect())
            .collect(),
    )
    .unwrap();

    match part {
        0 => {
            // for each coord, try each of the 8 directions, tallying how many we found
            let mut count = 0i64;
            let dirs = vec![
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            for y in 0..grid.h as i64 {
                for x in 0..grid.w as i64 {
                    count += dirs
                        .iter()
                        .filter(|dir| grid.has_str("XMAS", x, y, dir.0, dir.1))
                        .count() as i64
                }
            }
            count
        }

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4() {
        assert_eq!(solve(0), 2557);
        // assert_eq!(solve(1), 0);
    }
}
