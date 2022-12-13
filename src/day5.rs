pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let (raw_stacks, raw_moves) = input.split_once("\n\n").unwrap();
    let mut stacks = Vec::new();

    raw_stacks.lines().rev().skip(1).for_each(|l| {
        let line = l.to_string();
        let string: Vec<char> = line.chars().skip(1).step_by(4).collect();

        string
            .into_iter()
            .enumerate()
            .filter(|(_i, val)| !val.is_whitespace())
            .for_each(|(i, val)| {
                if stacks.len() <= i {
                    stacks.push(Vec::new());
                }
                stacks[i].push(val);
            });
    });

    let moves = raw_moves
        .lines()
        .map(|m| {
            m.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|d| d.parse().unwrap())
                .collect()
        })
        .collect();

    (stacks, moves)
}

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        for _ in 0..m[0] {
            let Some(tmp) = stacks[m[1] - 1].pop() else {
                continue;
            };
            stacks[m[2] - 1].push(tmp);
        }
    }

    stacks
        .into_iter()
        .map(|s| s.last().unwrap().clone())
        .collect()
}

pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        let len = stacks[m[1] - 1].len();
        let mut tmp: Vec<_> = stacks[m[1] - 1].drain(len - m[0]..).collect();
        stacks[m[2] - 1].append(&mut tmp);
    }

    stacks
        .into_iter()
        .map(|s| s.last().unwrap().clone())
        .collect()
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    use super::*;

    #[test]
    fn solve_day_5() {
        assert_eq!(part1(INPUT), "CMZ");
        assert_eq!(part2(INPUT), "MCD");
    }
}
