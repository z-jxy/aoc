use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut queue = vec![];

    let mut alternating = false;
    let mut fid = 0;

    let mut free_positions = vec![];

    input
        .bytes()
        .filter(|&c| c >= 48)
        .map(|c| c - 48)
        .for_each(|c| {
            alternating ^= true;
            let len = c as usize;
            if alternating {
                queue.extend(std::iter::repeat(fid).take(len));
                fid += 1;
            } else {
                let start = queue.len();
                queue.extend(std::iter::repeat(-1).take(len));
                free_positions.extend(start..start + len);
            }
        });

    for pos in free_positions {
        while queue.last() == Some(&-1) {
            queue.pop();
        }
        if queue.len() <= pos {
            break;
        }
        queue[pos] = queue.pop().unwrap();
    }

    queue
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, id)| acc + idx * *id as usize)
}

pub fn solve2_vec(input: &str) -> usize {
    let mut files = vec![(0, 0, 0); (input.len() / 2) + 1];
    let mut free = vec![];

    let mut fid = 0;
    let mut pos = 0;

    {
        let mut alternating = false;
        input
            .bytes()
            .filter(|&c| c >= 48)
            .map(|c| c - 48)
            .for_each(|c| {
                alternating ^= true;
                let len = c as usize;
                if alternating {
                    files[fid] = (fid, pos, len);
                    fid += 1;
                } else {
                    free.push((pos, len));
                }
                pos += len;
            });
    }
    let total_files = fid;

    while fid > 0 {
        fid -= 1;
        let (_, pos, size) = files[fid];
        for (idx, (start, length)) in free.iter().enumerate() {
            if start >= &pos {
                free.truncate(idx);
                break;
            }
            if size <= *length {
                *files.get_mut(fid).unwrap() = (fid, *start, size);

                if size == *length {
                    free.remove(idx);
                } else {
                    *free.get_mut(idx).unwrap() = (start + size, length - size);
                }

                break;
            }
        }
    }

    let mut total = 0;
    for (id, pos, size) in files[0..total_files].iter() {
        for i in *pos..*pos + *size {
            total += id * i;
        }
    }

    return total;
}

pub fn solve2_hashmap(input: &str) -> usize {
    let mut files = HashMap::new();
    let mut free = vec![];

    let mut fid = 0;
    let mut pos = 0;

    {
        let mut alternating = false;
        input
            .bytes()
            .filter(|&c| c >= 48)
            .map(|c| c - 48)
            .for_each(|c| {
                alternating ^= true;
                let len = c as usize;
                if alternating {
                    files.insert(fid, (pos, len));
                    fid += 1;
                } else {
                    free.push((pos, len));
                }
                pos += len;
            });
    }

    while fid > 0 {
        fid -= 1;
        let (pos, size) = files[&fid];
        for (idx, (start, length)) in free.iter().enumerate() {
            if start >= &pos {
                // free = free[..idx].to_vec();
                free.truncate(idx);
                break;
            }
            if size <= *length {
                files.entry(fid).and_modify(|x| *x = (*start, size));

                if size == *length {
                    free.remove(idx);
                } else {
                    *free.get_mut(idx).unwrap() = (start + size, length - size);
                }

                break;
            }
        }
    }

    let mut total = 0;
    for (id, (pos, size)) in files.iter() {
        for i in *pos..*pos + *size {
            total += id * i;
        }
    }

    return total;
}

#[aoc::main(09)]
fn main(input: &str) -> (usize, usize) {
    let (p1, p2) = (solve(&input), solve2_vec(input));

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2333133121414131402"#;

    #[test]
    fn test_p1() {
        let p1 = solve(EXAMPLE);
        assert_eq!(p1, 1928);
    }

    #[test]
    fn test_p2_hashmap_impl() {
        let p2 = solve2_hashmap(EXAMPLE);
        assert_eq!(p2, 2858);
    }

    #[test]
    fn test_p2_vec_impl() {
        let p2 = solve2_vec(EXAMPLE);
        assert_eq!(p2, 2858);
    }
}
