#[derive(Debug)]
enum IdRangeEntry {
    Left(u64),
    Right(u64),
}

impl IdRangeEntry {
    fn get_n(&self) -> &u64 {
        match self {
            Self::Left(n) => n,
            Self::Right(n) => n,
        }
    }
}

pub fn solve(part: u32) -> u64 {
    let mut ranges = vec![];
    let mut ids = vec![];
    let mut is_ids_section = false;

    std::fs::read_to_string("./src/day5_input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            if line == "" {
                is_ids_section = true
            } else if is_ids_section {
                ids.push(line.parse::<u64>().unwrap())
            } else {
                let (l, r) = line.split_once("-").unwrap();
                ranges.push((l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap() + 1));
            }
        });

    match part {
        // first pass, brute force check the ranges
        0 => ids
            .iter()
            .filter(|id| ranges.iter().any(|(low, high)| *id >= low && *id < high))
            .count() as u64,

        // how many fresh ids are there according to ranges (not caring about ids anymore)
        // this will need to be clever because the ranges are like 15 digit numbers and we can't just scan
        1 => {
            // plan: make a sorted list of range start and ends, because they overlap.
            // then scan through that list with an accumulator for range overlap amount, and sum up the size of the ranges when we're inside a range.
            let mut ordered: Vec<IdRangeEntry> = ranges
                .iter()
                .flat_map(|(l, r)| vec![IdRangeEntry::Left(*l), IdRangeEntry::Right(*r)])
                .collect();

            ordered.sort_by_key(|r| *r.get_n());

            let mut depth = 0;
            let mut count = 0u64;
            let mut current_id = 0u64;

            for r in ordered {
                let next_id = r.get_n();

                if depth > 0 {
                    count += next_id - current_id;
                }

                depth += match r {
                    IdRangeEntry::Left(_) => 1,
                    IdRangeEntry::Right(_) => -1,
                };

                current_id = *next_id;
            }

            count
        }

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5() {
        assert_eq!(solve(0), 698);
        assert_eq!(solve(1), 352807801032167);
    }
}
