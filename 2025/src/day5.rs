#[derive(Debug)]
enum IdRangeEntry {
    Left(u64),
    Right(u64),
}

impl IdRangeEntry {
    fn get_n(&self) -> u64 {
        match self {
            Self::Left(n) => *n,
            Self::Right(n) => *n,
        }
    }
}

fn load_data() -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = vec![];
    let mut ids = vec![];
    let mut is_ids_section = false;

    std::fs::read_to_string("./src/day5_input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            if line.is_empty() {
                is_ids_section = true
            } else if is_ids_section {
                ids.push(line.parse::<u64>().unwrap())
            } else {
                let (l, r) = line.split_once("-").unwrap();
                ranges.push((l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap() + 1)); // converting to non-inclusive range on the high side, so part 2 is simpler
            }
        });

    (ranges, ids)
}

pub fn solve(part: u64) -> u64 {
    match part {
        0 => solve_p0(),
        1 => solve_p1(),
        _ => panic!(),
    }
}

fn solve_p0() -> u64 {
    let (ranges, ids) = load_data();
    ids.iter()
        .filter(|id| ranges.iter().any(|(low, high)| *id >= low && *id < high))
        .count() as u64
}

fn solve_p1() -> u64 {
    let (ranges, _) = load_data();

    // plan: make a sorted list of range start and ends, because they overlap.
    // then scan through that list with an accumulator for range overlap amount, and sum up the size of the ranges when we're inside a range.
    let mut ordered: Vec<IdRangeEntry> = ranges
        .iter()
        .flat_map(|(l, r)| vec![IdRangeEntry::Left(*l), IdRangeEntry::Right(*r)])
        .collect();

    ordered.sort_by_key(|r| r.get_n());

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

        current_id = next_id;
    }

    count
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
