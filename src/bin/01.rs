pub fn part_one(input: &str) -> Option<u32> {
    input.split("\n\n").map(|elf| elf.lines().map(|n| n.parse::<u32>().unwrap()).sum()).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut x = input.split("\n\n").map(|elf| elf.lines().map(|n| n.parse::<u32>().unwrap()).sum()).collect::<Vec<u32>>();
    x.sort();
    Some(x.iter().rev().take(3).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
