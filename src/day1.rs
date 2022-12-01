pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .filter_map(|calory| calory.parse::<u32>().ok())
                .sum()
        })
        .collect()
}
pub fn part1(input: &str) -> Option<u32> {
    let calories = parse_input(input);

    calories.into_iter().max()
}

pub fn part2(input: &str) -> u32 {
    let mut calories = parse_input(input);

    calories.sort();
    calories.reverse();

    calories.into_iter().take(3).sum()
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    use super::{part1, part2};

    #[test]
    fn solve_day_1() {
        assert_eq!(part1(INPUT), Some(24000));
        assert_eq!(part2(INPUT), 45000);
    }
}
