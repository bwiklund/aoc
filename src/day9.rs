struct Point {
    x: i64,
    y: i64,
}

pub fn solve(part: u32) -> u64 {
    let red_tiles = std::fs::read_to_string("./src/day9_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    // sanity check for myself that there are no adjacent red tiles (at least not back to back in the order of the lines)
    for i in 0..red_tiles.len() {
        let p1 = &red_tiles[i];
        let p2 = &red_tiles[(i + red_tiles.len() - 1) % red_tiles.len()];
        if (p1.x - p2.x + p1.y - p2.y).abs() < 2 {
            panic!("oh no");
        }
    }

    let mut largest = 0;
    for i in 0..red_tiles.len() {
        for j in 0..i {
            let a = &red_tiles[i];
            let b = &red_tiles[j];

            let x1 = a.x.min(b.x);
            let y1 = a.y.min(b.y);
            let x2 = a.x.max(b.x);
            let y2 = a.y.max(b.y);

            let w = x2 - x1 + 1;
            let h = y2 - y1 + 1;

            let area = w * h;

            if part == 1 {
                // ok so making a grid and bucket filling it isn't gonna work because its huge (~10b cells). or maybe possible but really slow and not in the spirit of the question...
                // instead, i think the prob description says that a line connects the tiles, meaning they are never perfectly adjacent, meaning that i think? we can check if any of the lines intersect the candidate rectangles, and that will be enough to reject them? for our purposes, "intersecting" will mean one end of the line is inside the rect (and not just along the border)

                // n^3 baby lets gooooo
                if red_tiles.iter().enumerate().any(|(i, a)| {
                    // easy case. any vertex inside the borders
                    if a.x > x1 && a.x < x2 && a.y > y1 && a.y < y2 {
                        return true;
                    }

                    let j = (i + red_tiles.len() - 1) % red_tiles.len();
                    let b = &red_tiles[j];

                    // horizontal line through
                    if (a.x <= x1 && b.x >= x2) || (b.x <= x1 && a.x >= x2) {
                        if a.y > y1 && a.y < y2 {
                            return true;
                        }
                    }

                    // vertical line through
                    if (a.y <= y1 && b.y >= y2) || (b.y <= y1 && a.y >= y2) {
                        if a.x > x1 && a.x < x2 {
                            return true;
                        }
                    }

                    return false;
                }) {
                    continue;
                }
            }

            if area > largest {
                largest = area;
            }
        }
    }

    largest as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        assert_eq!(solve(0), 4763509452);
        assert_eq!(solve(1), 1516897893);
    }
}
