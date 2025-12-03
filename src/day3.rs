use std::fs::read_to_string;

pub fn solve(part: u32) -> u32 {
    read_to_string("./src/d3_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|batteries| {
            // pick biggest number except the last one
            // pick the biggest number after that number
            let n1_choices = &batteries[0..batteries.len().saturating_sub(1)];
            let n1_max = n1_choices.iter().max().cloned().unwrap_or(0);
            let n1_idx = n1_choices
                .iter()
                .position(|n| *n == n1_max)
                .unwrap_or_default(); // dumb second iteration

            let n2_choices = &batteries[n1_idx + 1..];
            let n2_max = n2_choices.iter().max().cloned().unwrap_or_default();

            dbg!(n1_choices[n1_idx], n2_max);

            n1_choices[n1_idx] * 10 + n2_max
        })
        .sum::<u32>()
}
