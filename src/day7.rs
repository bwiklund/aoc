use std::collections::{HashMap, HashSet};

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

    if part == 1 {
        // variant of part one, but the beams can count how many paths took them there, and add them up and multiply them and... stuff...
        let mut beam = HashMap::<usize, u64>::new();
        let mut split_count = 0;
        for row in env {
            for (idx, t) in row.iter().enumerate() {
                match t {
                    Tile::None => {}
                    Tile::Origin => {
                        beam.insert(idx, 1);
                    }
                    Tile::Splitter => {
                        if let Some(&tally) = beam.get(&idx) {
                            beam.insert(idx - 1, beam.get(&(idx - 1)).unwrap_or(&0) + tally);
                            beam.remove(&idx);
                            beam.insert(idx + 1, beam.get(&(idx + 1)).unwrap_or(&0) + tally);
                        }
                    }
                }
            }
        }
        return beam.iter().map(|(_, v)| v).sum();
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        assert_eq!(solve(0), 1499);
        assert_eq!(solve(1), 24743903847942);
    }
}
