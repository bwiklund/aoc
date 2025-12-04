struct Grid {
    cells: Vec<bool>,
    w: usize,
    h: usize,
}

impl Grid {
    pub fn from_vv(vv: Vec<Vec<bool>>) -> Option<Self> {
        // a real program would check for errors in row size but i am lazy
        Some(Grid {
            cells: vv.iter().flatten().copied().collect(),
            w: vv.len(),
            h: vv[0].len(),
        })
    }

    // convenience fn that returns false safely for oob entries
    fn is_occupied(&self, x: i32, y: i32) -> bool {
        if y < 0 || x < 0 || y >= self.h as i32 || x >= self.w as i32 {
            return false;
        }

        self.cells[y as usize * self.w + x as usize]
    }

    fn neighbor_count(&self, x: i32, y: i32) -> u32 {
        let mut count = 0;
        for iy in y - 1..=y + 1 {
            for ix in x - 1..=x + 1 {
                if ix == x && iy == y {
                    continue;
                }
                if self.is_occupied(ix, iy) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn solve(part: u32) -> u32 {
    let grid = Grid::from_vv(
        std::fs::read_to_string("./src/day4_input.txt")
            .unwrap()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| match ch {
                        '@' => true,
                        '.' => false,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
    .unwrap();

    match part {
        0 => (0..grid.h)
            .map(|iy| {
                (0..grid.w)
                    .filter(|&ix| {
                        grid.is_occupied(ix as i32, iy as i32)
                            && grid.neighbor_count(ix as i32, iy as i32) < 4
                    })
                    .count() as u32
            })
            .sum::<u32>(),

        1 => panic!("idk yet"),

        _ => panic!("there are no other parts"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4() {
        assert_eq!(solve(0), 1553);
    }
}
