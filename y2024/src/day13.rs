use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}

pub fn solve(part: u32) -> i64 {
    let re = Regex::new(
        r"^Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let machines = std::fs::read_to_string("./src/day13_input.txt")
        .unwrap()
        .split("\n\n")
        .map(|str| {
            let caps = re.captures(str).unwrap();
            Machine {
                a: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                b: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
                prize: (caps[5].parse().unwrap(), caps[6].parse().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    // dbg!(machines);

    match part {
        0 => machines
            .iter()
            .map(|m| {
                // doing linear algebra here by hand because it's just 2 rows...
                // eliminate 1,2
                let sub = m.a.1 as f64 / m.a.0 as f64;
                let mut rref = (
                    m.a.0 as f64,
                    m.b.0 as f64,
                    m.prize.0 as f64,
                    m.a.1 as f64 - m.a.0 as f64 * sub,
                    m.b.1 as f64 - m.b.0 as f64 * sub,
                    m.prize.1 as f64 - m.prize.0 as f64 * sub,
                );

                let sub = rref.1 as f64 / rref.4 as f64;
                rref.0 -= rref.3 * sub;
                rref.1 -= rref.4 * sub;
                rref.2 -= rref.5 * sub;

                let scale = rref.0;
                rref.0 /= scale;
                rref.1 /= scale;
                rref.2 /= scale;

                let scale = rref.4;
                rref.3 /= scale;
                rref.4 /= scale;
                rref.5 /= scale;

                fn round_close(n: f64) -> f64 {
                    if (n - n.round()).abs() < 0.000001 {
                        if n.round().abs() == 0.0 {
                            return 0.0;
                        }
                        return n.round();
                    } else {
                        return n;
                    }
                }

                fn is_int(n: f64) -> bool {
                    return n.round() == n;
                }

                rref.0 = round_close(rref.0);
                rref.1 = round_close(rref.1);
                rref.2 = round_close(rref.2);
                rref.3 = round_close(rref.3);
                rref.4 = round_close(rref.4);
                rref.5 = round_close(rref.5);

                if is_int(rref.2) && is_int(rref.5) {
                    let a_pushes = rref.2 as i32;
                    let b_pushes = rref.5 as i32;
                    if a_pushes > 100 || b_pushes > 100 {
                        return 0;
                    }
                    return a_pushes * 3 + b_pushes * 1;
                }

                0
                // dbg!(rref);
            })
            .sum::<i32>() as i64,

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
