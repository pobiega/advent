extern crate regex;

#[derive(Debug)]
pub struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn from_str(claim: &str, re: &regex::Regex) -> Claim {
        let caps = re.captures(claim).unwrap();
        let id = caps.get(1).unwrap().as_str().parse().unwrap();
        let left = caps.get(2).unwrap().as_str().parse().unwrap();
        let top = caps.get(3).unwrap().as_str().parse().unwrap();
        let width = caps.get(4).unwrap().as_str().parse().unwrap();
        let height = caps.get(5).unwrap().as_str().parse().unwrap();
        Claim {
            id,
            left,
            top,
            width,
            height,
        }
    }

    fn mark_on_fabric(&self, fabric: &mut [[u8; 1000]; 1000]) {
        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.height) {
                fabric[x][y] += 1;
            }
        }
    }

    fn overlaps(&self, fabric: &[[u8; 1000]; 1000]) -> bool {
        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.height) {
                if fabric[x][y] > 1 {
                    return true;
                }
            }
        }

        false
    }
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Claim> {
    let re = regex::Regex::new(r#"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"#).unwrap();

    input
        .lines()
        .map(|l| Claim::from_str(l.trim(), &re))
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Claim]) -> i32 {
    let mut fabric = [[0; 1000]; 1000];

    for claim in input {
        claim.mark_on_fabric(&mut fabric);
    }

    let mut count = 0;

    for row in fabric.iter() {
        for item in row.iter() {
            if *item > 1 {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day3, part2)]
pub fn part2(input: &[Claim]) -> Option<usize> {
    let mut fabric = [[0; 1000]; 1000];

    for claim in input {
        claim.mark_on_fabric(&mut fabric);
    }

    for claim in input {
        if claim.overlaps(&fabric) {
            return Some(claim.id);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};

    #[test]
    fn part1_test() {
        let claims = parse_input(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2",
        );

        assert_eq!(part1(&claims), 4)
    }

    #[test]
    fn part2_test() {
        let claims = parse_input(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2",
        );

        let value = part2(&claims);

        assert!(value.is_some());
        assert_eq!(value.unwrap(), 3);
    }
}
