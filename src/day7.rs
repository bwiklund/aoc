use std::collections::HashSet;

enum Tile {
    None,
    Origin,
    Splitter,
}

pub fn solve(part: u32) -> u64 {
    let env: Vec<Vec<Tile>> = std::fs::read_to_string("./src/day7_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| match ch {
                    '.' => Tile::None,
                    '^' => Tile::Splitter,
                    'S' => Tile::Origin,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    if part == 0 {
        let mut beam = HashSet::new();
        let mut split_count = 0;
        for row in env {
            for (idx, t) in row.iter().enumerate() {
                match t {
                    Tile::None => {}
                    Tile::Origin => {
                        beam.insert(idx);
                    }
                    Tile::Splitter => {
                        if beam.contains(&idx) {
                            beam.insert(idx - 1);
                            beam.remove(&idx);
                            beam.insert(idx + 1);
                            split_count += 1;
                        }
                    }
                }
            }
        }
        return split_count;
    }

    // // this might be totally inadquate to the numbers involved but lets see if this can work... just traverse and count...?
    if part == 1 {
        let x = env[0]
            .iter()
            .enumerate()
            .find(|(_, t)| matches!(t, Tile::Origin))
            .unwrap()
            .0;

        return rec_tachyon(&env, 1, x);

        fn rec_tachyon(env: &Vec<Vec<Tile>>, depth: usize, x: usize) -> u64 {
            if depth >= env.len() {
                return 0;
            }

            match env[depth][x] {
                Tile::Splitter => {
                    rec_tachyon(env, depth + 1, x - 1) + rec_tachyon(env, depth + 1, x + 1)
                }
                _ => rec_tachyon(env, depth + 1, x),
            }
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        assert_eq!(solve(0), 1499);
        assert_eq!(solve(1), 0);
    }
}
