#[derive(Copy, Clone)]
struct FileSpan {
    id: Option<i32>,
    len: usize,
}

type Disk = Vec<Option<i32>>;

pub fn solve(part: u32) -> i64 {
    let mut disk = parse_input_sparse().to_disk();

    match part {
        0 => {
            let mut first_free_idx = 0;
            // scan backwards from end
            for idx in (0..disk.len()).rev() {
                if disk[idx].is_some() {
                    // find the next free slot
                    while first_free_idx < disk.len() && disk[first_free_idx].is_some() {
                        first_free_idx += 1;
                    }
                    if first_free_idx >= disk.len() || first_free_idx >= idx {
                        break;
                    }
                    disk[first_free_idx] = disk[idx];
                    disk[idx] = None;
                }
            }
            checksum(disk)
        }

        1 => {
            // this could be smarter and faster but i wanna move on to a new problem
            let mut spans = parse_input_sparse(); // we need to remember original positions so we don't encounter them again and move them again
            let mut disk = spans.to_disk(); // initial state

            // backwards from known span positions
            spans.reverse();
            let mut head_pos = spans.iter().map(|s| s.len).sum();
            for span in spans {
                head_pos -= span.len;
                if span.id.is_some()
                    && let Some(dest_idx) = find_free_space(&mut disk, span.len, head_pos)
                {
                    move_file(&mut disk, head_pos, dest_idx, span.len);
                }
            }
            checksum(disk)
        }

        _ => unreachable!(),
    }
}

trait ToDisk {
    fn to_disk(&self) -> Disk;
}

impl ToDisk for Vec<FileSpan> {
    fn to_disk(&self) -> Disk {
        self.iter()
            .flat_map(|span| (0..span.len).map(|_| span.id))
            .collect()
    }
}

fn find_free_space(disk: &Disk, size_needed: usize, before_idx: usize) -> Option<usize> {
    let mut free_size = 0;
    for idx in 0..disk.len().min(before_idx) {
        if disk[idx].is_none() {
            free_size += 1;
            if free_size >= size_needed {
                return Some(idx - size_needed + 1);
            }
        } else {
            free_size = 0;
        }
    }
    None
}

fn move_file(disk: &mut Disk, from: usize, to: usize, len: usize) {
    for i in 0..len {
        disk[to + i] = disk[from + i];
        disk[from + i] = None;
    }
}

fn checksum(disk: Disk) -> i64 {
    disk.iter()
        .enumerate()
        .map(|(i, n)| match n {
            Some(n) => i as i64 * *n as i64,
            _ => 0,
        })
        .sum()
}

fn parse_input_sparse() -> Vec<FileSpan> {
    std::fs::read_to_string("./src/day9_input.txt")
        .unwrap()
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .enumerate()
        .map(|(idx, len)| FileSpan {
            id: match idx % 2 == 0 {
                true => Some((idx / 2) as i32),
                false => None,
            },
            len: len as usize,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        assert_eq!(solve(0), 6337367222422);
        assert_eq!(solve(1), 6361380647183);
    }
}
