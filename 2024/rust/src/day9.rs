#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
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

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2333133121414131402"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EXAMPLE), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&EXAMPLE), 2858);
    }
}
