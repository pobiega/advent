use std::collections::HashSet;
use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
pub fn part1(freqs: &[i32]) -> i32 {
    freqs.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(freqs: &[i32]) -> i32 {
    let mut freq: i32 = 0;
    let mut found = HashSet::<i32>::with_capacity(freqs.len());

    let mut it = freqs.into_iter().cycle();

    loop {
        if !found.insert(freq) {
            return freq;
        }
        match it.next() {
            Some(x) => freq += x,
            None => println!("{}", "Uh.. panic?"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        assert_eq!(part1(&[]), 0);
        assert_eq!(part1(&[1, 2, 3]), 6);
        assert_eq!(part1(&[1, -1, 3]), 3);
        assert_eq!(part1(&[-1, -2]), -3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&[1, -2, 3, 1, 1, -2]), 2);
        assert_eq!(part2(&[1, -1]), 0);
        assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
    }
}
