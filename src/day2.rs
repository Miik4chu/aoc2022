pub fn parse_input(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|l| -> (char, char) {
            let chars: Vec<char> = l
                .split_whitespace()
                .collect::<String>()
                .chars()
                .into_iter()
                .collect();
            let mut iter = chars.into_iter();

            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}
pub fn part1(input: &str) -> u32 {
    let sets = parse_input(input);
    sets.into_iter().fold(0, |pts, chars| -> u32 {
        let a = match chars {
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') | _ => 6,
        };

        let b = match chars {
            (_, 'X') => 1,
            (_, 'Y') => 2,
            (_, 'Z') => 3,
            _ => 0,
        };

        pts + a + b
    })
}

pub fn part2(input: &str) -> u32 {
    let sets = parse_input(input);
    sets.into_iter().fold(0, |pts, chars| -> u32 {
        let a = match chars {
            ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
            ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2,
            ('A', 'X') | ('B', 'Z') | ('C', 'Y') | _ => 3,
        };

        let b = match chars {
            (_, 'X') => 0,
            (_, 'Y') => 3,
            (_, 'Z') => 6,
            _ => 0,
        };

        pts + a + b
    })
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "A Y
B X
C Z
";

    use super::{part1, part2};

    #[test]
    fn solve_day_2() {
        assert_eq!(part1(INPUT), 15);
        assert_eq!(part2(INPUT), 12);
    }
}
