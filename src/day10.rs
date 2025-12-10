#[derive(Debug)]
struct Machine {
    lights_desired: Vec<bool>,
    buttons: Vec<Vec<u64>>,
    joltage: Vec<u64>,
}

struct MachineBitmask {
    lights_desired: u64,
    buttons: Vec<u64>,
}

fn parse_input() -> Vec<Machine> {
    std::fs::read_to_string("./src/day10_input.txt")
        .unwrap()
        .lines()
        .map(
            |l| match l.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
                [lights_desired_str, buttons_str @ .., joltage_str] => Machine {
                    lights_desired: lights_desired_str
                        .trim_matches(|c| c == '[' || c == ']')
                        .chars()
                        .map(|s| match s {
                            '.' => false,
                            '#' => true,
                            _ => panic!(),
                        })
                        .collect(),
                    buttons: buttons_str
                        .iter()
                        .map(|s| {
                            s.trim_matches(|c| c == '(' || c == ')')
                                .split(',')
                                .map(|s| s.parse::<u64>().unwrap())
                                .collect()
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
                .map(|m| MachineBitmask {
                    lights_desired: vec_bool_to_mask(&m.lights_desired),
                    buttons: m.buttons.iter().map(|b| vec_u64_to_mask(b)).collect(),
                })
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

        1 => {
            /*

            ok part 2
            - buttons can be pressed more than once
            - order still doesn't matter
            - a solution will be a count of presses per button, then summed
            - joltages in the input dont seem to have any (or many) zeros, so not worth ruling out buttons early
            - joltages are high enough that brute force will fail horribly. like 50^13 ~= 10^22 horribly
            - is this potentially a linear algebra problem? with the constraint that the vector solution can't have negative elements? ehhhhhhhh... if you can't negate rows then you're not gonna be able to get rid of any

            - another angle on the problem that might be a smaller space to solve in:
            - each joltage has a subset of buttons that can contribute to it.
            - so you know that the subset will be pushed 50 times, for example.
            - each joltage has a subset of buttons and a number of times they will all be pressed.
            - lets say each joltage has about a 5 button subset in the input data, and the average joltage is 50. order doesn't matter, only count, so that's still (on the order of) 50^5, which is 312,500,000 combos just for that button.
            - we can also put some trivial upper bounds on the MAX number of times each button can be pressed. which might make it (optomistically) more like 10^5 for a subset = 100,000. however brute forcing that would still be 100,000^13 = an absurdly large space of 10^65.

            - ok how about another angle
            - since buttons will need to be pushed (usually) many times, what if we combine buttons.
            - make a super-button of every button combined. use math (not iteration) to push it as many times as possible until the joltage has been filled (and overflowed some cols)
            - no this is nothing

            - ok how about iterating over this:
            - if you're lucky there will be at least one joltage that's only affected by one button. there's your answer for that button, press it that many times and move forward (keeping track by subtracting the joltages. when we're done it'll be zero across the board)
            - (repeat this step whenever you have a case of a button that is the only thing affecting a remaining joltage)
            - if this isn't the case then branching...? but i don't wanna

            */

            // ok well here's a stupid test: i can think of counterexamples to the method above but maybe the inputs are just like... nice

            // for m in machine {}

            0
        }

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10() {
        assert_eq!(solve(0), 411);
        assert_eq!(solve(1), 0);
    }
}
