use std::collections::HashSet;

pub fn solve(part: u32) -> i64 {
    let mut rules = vec![];
    let mut updates = vec![];
    let mut is_updates = false;
    std::fs::read_to_string("./src/day5_input.txt")
        .unwrap()
        .lines()
        .for_each(|l| {
            if l.is_empty() {
                is_updates = true;
                return;
            }

            if !is_updates {
                let (a, b) = l.split_once('|').unwrap();
                rules.push((a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
            } else {
                updates.push(
                    l.split(',')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }
        });

    fn is_in_order(update: &Vec<i64>, rules: &Vec<(i64, i64)>) -> bool {
        rules
            .iter()
            .filter(|r| update.contains(&r.0) && update.contains(&r.1))
            .all(|r| update.iter().position(|&x| x == r.0) < update.iter().position(|&x| x == r.1))
    }

    fn fix(update: &Vec<i64>, rules: &Vec<(i64, i64)>) -> Vec<i64> {
        // reduce to just the ones we care about
        let rules = rules
            .iter()
            .filter(|r| update.contains(&r.0) && update.contains(&r.1))
            .map(|r| *r)
            .collect::<Vec<_>>();

        let mut output = vec![];
        let mut remaining = update.iter().collect::<HashSet<_>>();

        // for each number in the input, do this:
        // if there are no rules saying it has to be after a number remaining in the input, append it to the output.

        // lots of copying here huh
        while remaining.len() > 0 {
            let mut found_p: Option<i64> = None;
            for &&p in remaining.iter() {
                if !rules
                    .iter()
                    .filter(|r| r.1 == p)
                    .any(|r| remaining.contains(&r.0))
                {
                    found_p = Some(p);
                }
            }
            if let Some(p) = found_p {
                output.push(p);
                remaining.remove(&p);
            } else {
                panic!("This shouldn't be possible");
            }
        }

        output
    }

    match part {
        0 => {
            // brute force first
            updates
                .iter()
                .filter(|update| is_in_order(update, &rules))
                .map(|update| update[update.len() / 2])
                .sum::<i64>()
        }

        1 => updates
            .iter()
            .filter(|update| !is_in_order(update, &rules))
            .map(|update| fix(update, &rules))
            .map(|update| update[update.len() / 2])
            .sum::<i64>(),

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5() {
        assert_eq!(solve(0), 5087);
        assert_eq!(solve(1), 4971);
    }
}
