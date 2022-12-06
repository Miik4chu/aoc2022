pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}
pub fn part1(input: &str) -> usize {
    let rucksacks = parse_input(input);
    rucksacks.into_iter().fold(0, |acc, rucksack| {
        let mid: usize = rucksack.len() / 2;
        let (s1, s2) = rucksack.split_at(mid);

        let priority = unique(s1).into_iter().fold(0, |acc2, c| {
            if unique(s2).into_iter().collect::<String>().contains(c) {
                acc2 + get_priority(c)
            } else {
                acc2
            }
        });

        priority + acc
    })
}

pub fn part2(input: &str) -> usize {
    let rucksacks = parse_input(input);
    let mut iter = rucksacks.iter();
    let mut priority = 0;

    while let Some(r1) = iter.next() {
        let Some(r2) = iter.next() else {
            continue;
        };
        let Some(r3) = iter.next() else {
            continue;
        };
        priority = priority
            + unique(r1).into_iter().fold(0, |acc, c| {
                if unique(r2).into_iter().collect::<String>().contains(c)
                    && unique(r3).into_iter().collect::<String>().contains(c)
                {
                    acc + get_priority(c)
                } else {
                    acc
                }
            });
    }

    priority
}

pub fn unique(s: &str) -> Vec<char> {
    let mut result: Vec<char> = s.chars().collect::<Vec<_>>();
    result.sort();
    result.dedup();

    result
}

pub fn get_priority(c: char) -> usize {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();

    alphabet.find(c).unwrap() + 1
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    use super::{part1, part2};

    #[test]
    fn solve_day_3() {
        assert_eq!(part1(INPUT), 157);
        assert_eq!(part2(INPUT), 70);
    }
}
