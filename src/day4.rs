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

    fn set(&mut self, x: i32, y: i32, val: bool) -> () {
        if y < 0 || x < 0 || y >= self.h as i32 || x >= self.w as i32 {
            return;
        }

        self.cells[y as usize * self.w + x as usize] = val;
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

    fn get_accessible_coords(&self) -> Vec<(usize, usize)> {
        (0..self.h)
            .flat_map(|iy| {
                (0..self.w).filter_map(move |ix| {
                    if self.is_occupied(ix as i32, iy as i32)
                        && self.neighbor_count(ix as i32, iy as i32) < 4
                    {
                        Some((ix, iy))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

pub fn solve(part: u32) -> usize {
    let mut grid = Grid::from_vv(
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
        0 => grid.get_accessible_coords().len(),

        1 => {
            let mut count = 0;
            loop {
                let coords = grid.get_accessible_coords();
                if coords.len() == 0 {
                    break;
                }
                count += coords.len();
                for (x, y) in coords {
                    grid.set(x as i32, y as i32, false);
                }
            }
            count
        }

        _ => panic!("there are no other parts"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4() {
        assert_eq!(solve(0), 1553);
        assert_eq!(solve(1), 8442);
    }
}
