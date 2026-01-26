use std::collections::{HashSet, VecDeque};

struct Grid<T> {
    cells: Vec<Vec<T>>,
    w: i32,
    h: i32,
}

impl<T> Grid<T> {
    fn get(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            None
        } else {
            Some(&self.cells[y as usize][x as usize])
        }
    }

    fn set(&mut self, x: i32, y: i32, val: T) {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            return;
        } else {
            self.cells[y as usize][x as usize] = val;
        }
    }
}

struct Region {
    cells: HashSet<(i32, i32)>,
    borders: HashSet<(i32, i32, i32, i32)>,
}

pub fn solve(part: u32) -> i64 {
    let input = std::fs::read_to_string("./src/day12_input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut grid = Grid {
        w: input[0].len() as i32,
        h: input.len() as i32,
        cells: input,
    };

    let mut regions = vec![];
    for y in 0..grid.h {
        for x in 0..grid.w {
            let search = *grid.get(x, y).unwrap();
            if search == '.' {
                continue;
            }

            let mut cells = HashSet::new();
            let mut bucket_queue = VecDeque::new();

            bucket_queue.push_back((x, y));
            while let Some((x, y)) = bucket_queue.pop_front() {
                if grid.get(x, y) == Some(&search) {
                    cells.insert((x, y));
                    grid.set(x, y, '.');

                    bucket_queue.push_back((x + 1, y));
                    bucket_queue.push_back((x, y + 1));
                    bucket_queue.push_back((x - 1, y));
                    bucket_queue.push_back((x, y - 1));
                }
            }

            let borders = cells
                .iter()
                .flat_map(|(x, y)| {
                    let mut cell_borders = vec![];
                    let mut add_border_if_exists = |&x, &y, dx, dy| {
                        if !cells.contains(&(x + dx, y + dy)) {
                            cell_borders.push((x, y, dx, dy))
                        }
                    };
                    add_border_if_exists(x, y, 1, 0);
                    add_border_if_exists(x, y, 0, 1);
                    add_border_if_exists(x, y, -1, 0);
                    add_border_if_exists(x, y, 0, -1);
                    cell_borders
                })
                .collect();

            if cells.len() > 0 {
                regions.push(Region {
                    cells: cells,
                    borders: borders,
                });
            }
        }
    }

    match part {
        0 => regions
            .iter()
            .map(|r| {
                let area = r.cells.len();
                let perimeter = r.borders.len();
                area as i64 * perimeter as i64
            })
            .sum(),

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12() {
        assert_eq!(solve(0), 1573474);
        assert_eq!(solve(1), 0);
    }
}
