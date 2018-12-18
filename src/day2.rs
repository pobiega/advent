#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let codes = input.lines();

    let mut twos = 0;
    let mut threes = 0;

    for code in codes {
        let mut has_two = false;
        let mut has_three = false;

        for c in code.chars() {
            let count = code.matches(c).count();
            match count {
                2 => has_two = true,
                3 => has_three = true,
                _ => {}
            }
            if has_two && has_three {
                break;
            }
        }

        if has_two {
            twos += 1
        }
        if has_three {
            threes += 1
        }
    }

    twos * threes
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    let codes: Vec<&str> = input.lines().collect();

    for code1 in codes.iter() {
        for code2 in codes.iter() {
            if let Some(same) = part2_differ(&code1, &code2) {
                return same;
            }
        }
    }
    "Hello".to_string()
}

fn part2_differ(a: &str, b: &str) -> Option<String> {
    let mut differences = 0;
    let mut same = String::new();

    for (c1, c2) in a.chars().zip(b.chars()) {
        if c1 == c2 {
            same.push(c1);
        } else {
            differences += 1;
        }
    }

    if differences == 1 {
        Some(same)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, part2_differ};

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"
            ),
            12
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
            ),
            "fgij"
        );
    }

    #[test]
    fn part2_differ_test() {
        let test = part2_differ("abcde", "axcde");

        match test {
            Some(x) => assert_eq!(x, "acde"),
            None => panic!("part2_differ returned None on known inputs"),
        }
    }
}
