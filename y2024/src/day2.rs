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
                let diffs: Vec<i64> = r.windows(2).map(|w| w[1] - w[0]).collect();
                diffs.iter().all(|d| (1..=3).contains(&d.abs()))
                    && (diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0))
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
