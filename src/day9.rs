struct Point {
    x: i64,
    y: i64,
}

pub fn solve(part: u32) -> u64 {
    let red_tiles = std::fs::read_to_string("./src/day9_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            Point {
                x: x.parse::<i64>().unwrap(),
                y: y.parse::<i64>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    match part {
        0 => {
            let mut largest = 0;
            for i in 0..red_tiles.len() {
                for j in 0..i {
                    let a = &red_tiles[i];
                    let b = &red_tiles[j];

                    let w = (a.x - b.x).abs() + 1;
                    let h = (a.y - b.y).abs() + 1;

                    let area = w * h;

                    if area > largest {
                        largest = area;
                    }
                }
            }

            largest as u64
        }

        1 => 0,

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        assert_eq!(solve(0), 4763509452);
        // assert_eq!(solve(1), 0);
    }
}
