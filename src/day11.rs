use std::collections::HashMap;

#[derive(Debug)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl Device {
    pub fn parse(s: &str) -> Result<Device, &str> {
        let (name, outputs) = s.split_once(": ").unwrap();
        Ok(Device {
            name: name.to_string(),
            outputs: outputs.split_ascii_whitespace().map(Into::into).collect(),
        })
    }
}

pub fn solve(part: u32) -> u64 {
    let devices: HashMap<String, Device> = std::fs::read_to_string("./src/day11_input.txt")
        .unwrap()
        .lines()
        .filter_map(|s| Device::parse(s).ok())
        .map(|d| (d.name.clone(), d))
        .collect();

    match part {
        0 => {
            dbg!(&devices);
            // the problem doesn't say no loops, but if there were loops then the answer would be infinity. so there must not be.
            // let seen = HashSet::<String>::new();
            fn search(devices: &HashMap<String, Device>, id: &str) -> u64 {
                match id {
                    "out" => 1,
                    _ => devices[id]
                        .outputs
                        .iter()
                        .map(|oid| search(devices, oid))
                        .sum::<u64>(),
                }
            }
            search(&devices, "you")
        }

        1 => 0,

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
