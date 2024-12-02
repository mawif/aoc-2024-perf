use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    let mut l: i64 = 0;
    let mut r: i64 = 0;
    let mut current = &mut l;
    for b in input.as_bytes().iter() {
        match b {
            b' ' => {
                current = &mut r;
            }
            b'\n' => {
                left.push(l);
                l = 0;
                current = &mut l;
                right.push(r);
                r = 0;
            }
            b'\r' => continue,
            _ => {
                *current *= 10;
                *current += (b - '0' as u8) as i64;
            }
        }
    }
    // aoc framework appears to strip trailing whitespace,
    // so no matter how many extra line endings one adds to
    // the file, final line won't have a '\n' to trigger
    // that branch of the match, which is important.
    if l != 0 {
        left.push(l);
        right.push(r);
    }
    left.sort_unstable();
    right.sort_unstable();
    std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let mut left = Vec::with_capacity(1000);
    let mut right_counts = HashMap::with_capacity(1000);
    let mut l: i64 = 0;
    let mut r: i64 = 0;
    let mut current = &mut l;
    for b in input.as_bytes().iter() {
        match b {
            b' ' => {
                current = &mut r;
            }
            b'\n' => {
                left.push(l);
                l = 0;
                current = &mut l;
                let stat = right_counts.entry(r).or_insert(0);
                *stat += 1;
                r = 0;
            }
            b'\r' => continue,
            _ => {
                *current *= 10;
                *current += (b - '0' as u8) as i64;
            }
        }
    }
    if l != 0 {
        left.push(l);
        let stat = right_counts.entry(r).or_insert(0);
        *stat += 1;
    }
    let mut result = 0;
    for l in left.iter() {
        if let Some(r) = right_counts.get(l) {
            result += l * r;
        }
    }
    result
}
