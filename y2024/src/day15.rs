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

impl std::fmt::Debug for Grid<Option<Thing>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                let ch = match self.get(x, y).unwrap() {
                    Some(Thing::Wall) => '#',
                    Some(Thing::Robot) => '@',
                    Some(Thing::Barrel) => 'O',
                    None => '.',
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
enum Thing {
    Robot,
    Barrel,
    Wall,
}

fn parse_input() -> (Grid<Option<Thing>>, Vec<(i32, i32)>) {
    let txt = std::fs::read_to_string("./src/day15_input.txt").unwrap();
    let mut lines = txt.lines();

    let mut map_lines: Vec<Vec<Option<Thing>>> = vec![];
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        map_lines.push(
            line.chars()
                .map(|ch| match ch {
                    '#' => Some(Thing::Wall),
                    '@' => Some(Thing::Robot),
                    'O' => Some(Thing::Barrel),
                    _ => None,
                })
                .collect(),
        );
    }
    let grid = Grid {
        w: map_lines[0].len() as i32,
        h: map_lines.len() as i32,
        cells: map_lines,
    };

    let moves = lines
        .flat_map(|l| {
            l.chars().map(|ch| match ch {
                '>' => (1, 0),
                '<' => (-1, 0),
                'v' => (0, 1),
                '^' => (0, -1),
                _ => panic!("Invalid move direction"),
            })
        })
        .collect();

    (grid, moves)
}

fn move_robot(grid: &mut Grid<Option<Thing>>, (dx, dy): (i32, i32)) {
    let (x, y) = grid
        .cells
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, maybe_thing)| match maybe_thing {
                    Some(Thing::Robot) => Some((x as i32, y as i32)),
                    _ => None,
                })
        })
        .next()
        .unwrap();

    // scan out until we find an empty cell. if there is one, move everything we scanned over by 1. else stop
    let mut scan_x = x;
    let mut scan_y = y;
    for _ in 0.. {
        scan_x += dx;
        scan_y += dy;
        match grid.get(scan_x, scan_y) {
            None | Some(Some(Thing::Wall)) => {
                break;
            }
            Some(None) => {
                while (scan_x, scan_y) != (x, y) {
                    grid.set(
                        scan_x,
                        scan_y,
                        grid.get(scan_x - dx, scan_y - dy).unwrap().clone(),
                    );
                    scan_x -= dx;
                    scan_y -= dy;
                }
                grid.set(x, y, None);
                break;
            }
            _ => continue,
        }
    }
}

fn gps_coord(x: i32, y: i32) -> i32 {
    x + y * 100
}

pub fn solve(part: u32) -> i64 {
    let (mut grid, moves) = parse_input();

    match part {
        0 => {
            for m in moves {
                move_robot(&mut grid, m);
                // dbg!(&grid);
                // std::thread::sleep(Duration::from_millis(16));
            }
            grid.cells
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().map(move |(x, thing)| match thing {
                        Some(Thing::Barrel) => gps_coord(x as i32, y as i32),
                        _ => 0,
                    })
                })
                .sum::<i32>() as i64
        }

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15() {
        assert_eq!(solve(0), 1429911);
        // assert_eq!(solve(1), 0);
    }
}
