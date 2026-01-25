use std::collections::HashSet;

struct Grid {
    cells: Vec<Vec<i32>>,
    w: i32,
    h: i32,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            None
        } else {
            Some(self.cells[y as usize][x as usize])
        }
    }
}

fn traverse(
    grid: &Grid,
    dest_counter: &mut HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    last_elevation: i32,
) {
    let next_elev = last_elevation + 1;
    if grid.get(x, y) == Some(next_elev) {
        if next_elev == 9 {
            dest_counter.insert((x, y));
            return;
        }
        traverse(grid, dest_counter, x + 1, y, next_elev);
        traverse(grid, dest_counter, x - 1, y, next_elev);
        traverse(grid, dest_counter, x, y + 1, next_elev);
        traverse(grid, dest_counter, x, y - 1, next_elev);
    }
}

pub fn solve(part: u32) -> i32 {
    let input = std::fs::read_to_string("./src/day10_input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let grid = Grid {
        w: input[0].len() as i32,
        h: input.len() as i32,
        cells: input,
    };

    match part {
        0 => {
            let mut score = 0;
            for y in 0..grid.h {
                for x in 0..grid.w {
                    let mut counter = HashSet::new();
                    traverse(&grid, &mut counter, x, y, -1);
                    score += counter.len() as i32
                }
            }
            score
        }

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10() {
        assert_eq!(solve(0), 816);
        // assert_eq!(solve(1), 0);
    }
}
