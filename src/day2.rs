#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {

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

fn part2(input: &str) -> String {
    "Hello".to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        assert_eq!(part1(
"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"
        ), 12);
    }

    fn part2_test() {
        assert_eq!(part2(
"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
        ), "fgij");
    }
}