use std::collections::HashMap;

pub fn solve(part: u32) -> i64 {
    let (mut l1, mut l2): (Vec<i64>, Vec<i64>) = std::fs::read_to_string("./src/day1_input.txt")
        .unwrap()
        .lines()
        .filter_map(|l| {
            let mut parts = l.split_ascii_whitespace();
            let a: i64 = parts.next()?.parse().ok()?;
            let b: i64 = parts.next()?.parse().ok()?;
            Some((a, b))
        })
        .unzip();

    l1.sort_unstable();
    l2.sort_unstable();

    match part {
        0 => l1.iter().zip(l2).map(|(a, b)| (a - b).abs()).sum(),

        1 => {
            let l2counts = l2.iter().fold(HashMap::<i64, i64>::new(), |mut acc, &n| {
                *acc.entry(n).or_insert(0) += 1;
                acc
            });
            l1.iter().map(|n| n * l2counts.get(n).unwrap_or(&0)).sum()
        }

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(solve(0), 2742123);
        assert_eq!(solve(1), 21328497);
    }
}
