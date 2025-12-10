#[derive(Debug)]
struct Machine {
    lights_desired: u64,
    buttons: Vec<u64>,
    joltage: Vec<u64>,
}

fn parse_input() -> Vec<Machine> {
    std::fs::read_to_string("./src/day10_input.txt")
        .unwrap()
        .lines()
        .map(
            |l| match l.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
                [lights_desired_str, buttons_str @ .., joltage_str] => Machine {
                    lights_desired: vec_bool_to_mask(
                        &lights_desired_str
                            .trim_matches(|c| c == '[' || c == ']')
                            .chars()
                            .map(|s| match s {
                                '.' => false,
                                '#' => true,
                                _ => panic!(),
                            })
                            .collect(),
                    ),
                    buttons: buttons_str
                        .iter()
                        .map(|s| {
                            vec_u64_to_mask(
                                &s.trim_matches(|c| c == '(' || c == ')')
                                    .split(',')
                                    .map(|s| s.parse::<u64>().unwrap())
                                    .collect(),
                            )
                        })
                        .collect(),
                    joltage: joltage_str
                        .trim_matches(|c| c == '{' || c == '}')
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect(),
                },
                _ => panic!(),
            },
        )
        .collect()
}

fn vec_bool_to_mask(xs: &Vec<bool>) -> u64 {
    if xs.len() > 64 {
        panic!("Won't fit in a mask!");
    }

    xs.iter().enumerate().map(|(i, b)| (*b as u64) << i).sum()
}

fn vec_u64_to_mask(xs: &Vec<u64>) -> u64 {
    xs.iter()
        .map(|x| {
            1u64.checked_shl(*x as u32)
                .expect("Doesn't fit in bitmask!")
        })
        .sum()
}

pub fn solve(part: u32) -> u64 {
    let machines: Vec<Machine> = parse_input();

    match part {
        0 => {
            // dbg!(machines);
            // dbg!(machines.iter().map(|m| m.buttons.len()).max());
            // dbg!(vec_bool_to_mask(&machines[0].lights_desired));
            /*

            ok so for trying to find the button combos to get the lights how we want, a few facts:
            - pushing a button twice in a row is a no-op, so we will never do that, which constrains the solution space
            - finding ourselves in a state we've been at before is a fail state, so maybe track a hashset of seen states (in a bit mask? they would all fit in 16 bits) (this might not be worthwhile given next thoughts)
            - and in fact, since all we're doing is flipping bits, i believe there's no reason to push the same button twice ever
            - so each machine's problem search space is N!. the largest set of buttons in the input is 13, and 13! = 6,227,020,800. so probably seconds per machine that size...
            - the buttons can also be bitmasks and flip the lights super fast.
            - ok actually not only does the order of one buttons push not matter, the order of buttons doesn't matter either, just which were pressed. which makes the problem space 2^N (each solution is just whether each button was pressed 0 or 1 times)
            - 2^13 is only 8,192, which is about 1m times smaller to think about

            */

            machines
                .iter()
                .map(|m| {
                    let mask_goal = m.lights_desired;
                    (0..(2u64 << m.buttons.len()))
                        .filter_map(|buttons_on| {
                            let mut lights = 0u64;
                            let mut flip_count = 0;
                            for (i, btn_mask) in m.buttons.iter().enumerate() {
                                if buttons_on & (1 << i) > 0 {
                                    lights ^= btn_mask;
                                    flip_count += 1;
                                }
                            }
                            match mask_goal == lights {
                                true => Some(flip_count),
                                false => None,
                            }
                        })
                        .min()
                        .expect("Couldn't find a solution.")
                })
                .sum()
        }

        1 => 0,

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10() {
        assert_eq!(solve(0), 411);
        // assert_eq!(solve(1), 0);
    }
}
