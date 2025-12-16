use std::ops::RangeInclusive;

struct Ctx {
    chars: Vec<char>,
    idx: usize,
}

impl Ctx {
    fn is_done(&self) -> bool {
        self.idx >= self.chars.len()
    }

    fn accept(&mut self, ex: char) -> Option<char> {
        if let Some(ch) = self.chars.get(self.idx)
            && *ch == ex
        {
            self.idx += 1;
            Some(*ch)
        } else {
            None
        }
    }

    fn accept_str<'a>(&mut self, ex: &'a str) -> Option<&'a str> {
        let back = self.idx;
        if ex.chars().all(|ch| self.accept(ch).is_some()) {
            Some(ex)
        } else {
            self.idx = back;
            None
        }
    }

    fn accept_range(&mut self, range: RangeInclusive<char>) -> Option<char> {
        if let Some(ch) = self.chars.get(self.idx)
            && range.contains(ch)
        {
            self.idx += 1;
            Some(*ch)
        } else {
            None
        }
    }

    fn accept_number(&mut self) -> Option<i64> {
        // first digit required
        if let Some(d) = self.accept_range('0'..='9').and_then(|ch| ch.to_digit(10)) {
            // consume 2 more optional digits
            let mut n = d;
            for _ in 0..2 {
                if let Some(d) = self.accept_range('0'..='9').and_then(|ch| ch.to_digit(10)) {
                    n = n * 10 + d;
                }
            }
            Some(n.into())
        } else {
            None
        }
    }
}

fn parse(ctx: &mut Ctx, consider_dos: bool) -> i64 {
    let mut accum = 0;
    let mut enabled = true;
    while !ctx.is_done() {
        // any failure leaves idx unmodified and starts checking from the top with the last failed index. not good enough for a real language but good enough for the problem as stated so far...

        let old_idx = ctx.idx;

        if consider_dos && ctx.accept_str("do()").is_some() {
            enabled = true;
        } else if consider_dos && ctx.accept_str("don't()").is_some() {
            enabled = false;
        } else if ctx.accept_str("mul(").is_some()
            && let Some(n1) = ctx.accept_number()
            && ctx.accept(',').is_some()
            && let Some(n2) = ctx.accept_number()
            && ctx.accept(')').is_some()
        {
            if enabled {
                accum += n1 * n2
            }
        } else {
            if ctx.idx == old_idx {
                ctx.idx += 1;
            }
        }
    }
    accum
}

pub fn solve(part: u32) -> i64 {
    let mut ctx = Ctx {
        chars: std::fs::read_to_string("./src/day3_input.txt")
            .unwrap()
            .chars()
            .collect(),
        idx: 0,
    };

    match part {
        0 => parse(&mut ctx, false),

        1 => parse(&mut ctx, true),

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        assert_eq!(solve(0), 170778545);
        assert_eq!(solve(1), 82868252);
    }
}
