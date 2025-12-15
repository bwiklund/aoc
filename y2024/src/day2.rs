pub fn solve(part: u32) -> u64 {
    let reports = std::fs::read_to_string("./src/day2_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    match part {
        0 => reports
            .iter()
            .filter(|r| {
                let mut going_up: Option<bool> = None;
                for i in 1..r.len() {
                    let diff = r[i] - r[i - 1];
                    if diff.abs() < 1 || diff.abs() > 3 {
                        return false;
                    }
                    if going_up.is_none() {
                        going_up = Some(diff > 0);
                    } else if going_up != Some(diff > 0) {
                        return false;
                    }
                }
                true
            })
            .count() as u64,

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        assert_eq!(solve(0), 334);
        // assert_eq!(solve(1), 0);
    }
}
