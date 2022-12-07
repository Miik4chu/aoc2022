use std::ops::RangeInclusive;

pub fn parse_input(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input
        .lines()
        .map(|l| match l.split_once(',') {
            Some((range1, range2)) => (parse_range(range1), parse_range(range2)),
            None => {
                panic!("?")
            }
        })
        .collect()
}

pub fn parse_range(range: &str) -> RangeInclusive<usize> {
    match range.split_once('-') {
        Some((start, end)) => start.parse().unwrap()..=end.parse().unwrap(),
        None => {
            panic!("help")
        }
    }
}

pub fn part1(input: &str) -> usize {
    let ranges = parse_input(input);

    ranges
        .into_iter()
        .filter(|r| {
            let (range1, range2) = r;

            // RangeInclusive does not implement Copy trait so we need to clone it
            // and it also does not implement len() just like its counter part Range so we use size_hint
            if range1.clone().into_iter().size_hint() < range2.clone().into_iter().size_hint() {
                fully_contains(range1, range2)
            } else {
                fully_contains(range2, range1)
            }
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let ranges = parse_input(input);

    ranges
        .into_iter()
        .filter(|r| {
            let (range1, range2) = r;

            if range1.clone().into_iter().size_hint() < range2.clone().into_iter().size_hint() {
                overlaps(range1, range2)
            } else {
                overlaps(range2, range1)
            }
        })
        .count()
}

pub fn fully_contains(range1: &RangeInclusive<usize>, range2: &RangeInclusive<usize>) -> bool {
    range2.contains(range1.start()) && range2.contains(range1.end())
}

pub fn overlaps(range1: &RangeInclusive<usize>, range2: &RangeInclusive<usize>) -> bool {
    range2.contains(range1.start()) || range2.contains(range1.end())
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    use super::{part1, part2};

    #[test]
    fn solve_day_3() {
        assert_eq!(part1(INPUT), 2);
        assert_eq!(part2(INPUT), 4);
    }
}
