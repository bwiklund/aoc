use std::collections::HashSet;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Start,
    Wall,
}

struct Grid {
    rows: Vec<Vec<Tile>>,
    w: i64,
    h: i64,
    guard_x: i64,
    guard_y: i64,
    guard_dir: i64,
    visited: HashSet<(i64, i64)>,
}

impl Grid {
    fn new(str: &str) -> Self {
        let mut rows: Vec<Vec<Tile>> = str
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| match ch {
                        '.' => Tile::Empty,
                        '^' => Tile::Start,
                        '#' => Tile::Wall,
                        _ => panic!("bad input"),
                    })
                    .collect()
            })
            .collect();
        let (guard_x, guard_y) = rows
            .iter()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter().enumerate().filter_map(move |(x, cell)| {
                    if *cell == Tile::Start {
                        Some((x as i64, y as i64))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>()[0];
        // rows[guard_y as usize][guard_x as usize] = Tile::Empty;
        let w = rows[0].len() as i64;
        let h = rows.len() as i64;
        let mut visited = HashSet::new();
        visited.insert((guard_x, guard_y));
        Self {
            rows,
            w,
            h,
            guard_x,
            guard_y,
            guard_dir: 0,
            visited,
        }
    }

    fn get(&self, x: i64, y: i64) -> Option<&Tile> {
        self.rows
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }

    fn advance(&mut self) -> bool {
        let (dx, dy) = self.guard_dir_to_vel();
        if let Some(Tile::Wall) = self.get(self.guard_x + dx, self.guard_y + dy) {
            self.guard_dir = (self.guard_dir + 1).rem_euclid(4);
        } else {
            self.guard_x += dx;
            self.guard_y += dy;
        }

        // return whether we're still in bounds
        if self.guard_x >= 0 && self.guard_x < self.w && self.guard_y >= 0 && self.guard_y < self.h
        {
            self.visited.insert((self.guard_x, self.guard_y));
            true
        } else {
            false
        }
    }

    fn guard_dir_to_vel(&self) -> (i64, i64) {
        let (dx, dy) = match self.guard_dir.rem_euclid(4) {
            0 => (0, -1),
            1 => (1, 0),
            2 => (0, 1),
            3 => (-1, 0),
            _ => unreachable!(),
        };
        (dx, dy)
    }
}

pub fn solve(part: u32) -> i64 {
    let mut grid = Grid::new(
        std::fs::read_to_string("./src/day6_input.txt")
            .unwrap()
            .as_str(),
    );

    match part {
        0 => {
            while grid.advance() {}
            grid.visited.len() as i64
        }

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
