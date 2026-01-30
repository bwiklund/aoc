use std::collections::HashSet;

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
                    Some(Thing::BarrelLeft) => '[',
                    Some(Thing::BarrelRight) => ']',
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
    Wall,
    Barrel,
    // for part 2 this is good enough. i don't need the exercise of doing this with more complex objects because we already made sunshine heavy industries.
    BarrelLeft,
    BarrelRight,
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

// if returns false, a block is blocked and nothing can move. else all the blocks returned are valid to move
fn gather_affected_blocks(
    grid: &mut Grid<Option<Thing>>,
    blocks: &mut HashSet<(i32, i32)>,
    (x, y): (i32, i32),
    (dx, dy): (i32, i32),
) -> bool {
    let mut scan_x = x;
    let mut scan_y = y;
    for _ in 0.. {
        scan_x += dx;
        scan_y += dy;
        match grid.get(scan_x, scan_y) {
            None | Some(Some(Thing::Wall)) => {
                return false;
            }
            Some(None) => {
                return true;
            }
            _ => {
                blocks.insert((scan_x, scan_y));
            }
        }
    }
    false
}

fn move_blocks(grid: &mut Grid<Option<Thing>>, blocks: HashSet<(i32, i32)>, (dx, dy): (i32, i32)) {
    let move_things: Vec<_> = blocks
        .iter()
        .map(|&(x, y)| {
            let thing = *grid.get(x, y).unwrap();
            grid.set(x, y, None);
            ((x, y), thing)
        })
        .collect();

    for ((x, y), thing) in move_things {
        grid.set(x + dx, y + dy, thing);
    }
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
    let mut blocks = HashSet::new();
    blocks.insert((x, y)); // the robot too
    let can_move = gather_affected_blocks(grid, &mut blocks, (x, y), (dx, dy));
    if can_move {
        move_blocks(grid, blocks, (dx, dy));
    }
}

fn gps_coord(x: i32, y: i32) -> i32 {
    x + y * 100
}

fn gps_checksum(grid: Grid<Option<Thing>>) -> i32 {
    grid.cells
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, thing)| match thing {
                Some(Thing::Barrel) => gps_coord(x as i32, y as i32),
                _ => 0,
            })
        })
        .sum::<i32>()
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
            gps_checksum(grid) as i64
        }

        1 => {
            // let grid = inflate_grid
            0
        },

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
