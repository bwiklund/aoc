use std::collections::HashMap;

pub fn solve(part: u32) -> u64 {
    let stones = std::fs::read_to_string("./src/day11_input.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    match part {
        0 => stones.iter().map(|s| blink_rec(*s, 25)).sum(),

        1 => stones
            .iter()
            .map(|s| blink_rec_memo(*s, 75, &mut HashMap::new()))
            .sum(),

        _ => unreachable!(),
    }
}

fn blink_rec(stone: u64, depth_left: u32) -> u64 {
    if depth_left == 0 {
        return 1;
    }
    let (next, extra) = blink(stone);
    return blink_rec(next, depth_left - 1)
        + extra.map(|e| blink_rec(e, depth_left - 1)).unwrap_or(0);
}

fn blink_rec_memo(stone: u64, depth_left: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if depth_left == 0 {
        return 1;
    }

    if let Some(res) = memo.get(&(stone, depth_left)) {
        return *res;
    }

    let (next, extra) = blink(stone);
    let res = blink_rec_memo(next, depth_left - 1, memo)
        + extra
            .map(|e| blink_rec_memo(e, depth_left - 1, memo))
            .unwrap_or(0);

    memo.insert((stone, depth_left), res);

    res
}

fn blink(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }
    let digit_count = (stone as f64).log10().floor() as u32 + 1;
    if digit_count % 2 == 0 {
        let divisor = 10u64.pow(digit_count / 2);
        return (stone / divisor, Some(stone % divisor));
    }
    return (stone.checked_mul(2024).unwrap(), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11() {
        assert_eq!(solve(0), 216996);
        assert_eq!(solve(1), 257335372288947);
    }
}
