use std::fs::read_to_string;

pub fn solve(part: u32) -> u64 {
    fn max_first(v: &[u64]) -> (usize, u64) {
        // fixme scan once not twice
        let max = v.iter().max().cloned().unwrap_or(0);
        let idx = v.iter().position(|n| *n == max).unwrap_or_default();
        (idx, max)
    }

    read_to_string("./src/d3_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|batteries| {
            // pick the biggest number available, ignoring enough at the end to ensure we have enough digits left
            let n = match part {
                0 => 2,
                1 => 12,
                _ => panic!(),
            };

            let mut digit_idxs = vec![];
            let mut min_idx = 0;
            for i in 0..n {
                let choices = &batteries[min_idx..batteries.len().saturating_sub(n - i - 1)];
                let (idx, _) = max_first(choices);
                digit_idxs.push(idx + min_idx);
                min_idx += idx + 1;
            }

            digit_idxs
                .iter()
                .enumerate()
                .map(|(digit, &idx)| batteries[idx] * 10u64.pow((n - digit - 1) as u32))
                .sum::<u64>()
        })
        .sum()
}
