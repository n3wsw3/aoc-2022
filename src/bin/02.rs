const BEATS: &[u32] = &[2, 0, 1];

fn result(hand: u32, other: u32) -> u32 {
    match (hand, other) {
        _ if BEATS[hand as usize]  == other => 6,
        _ if BEATS[other as usize] == hand  => 0,
        _                                   => 3,
    }
}

fn char_offset(str: &str, char: char) -> u32 {
    str.chars().nth(0).unwrap() as u32 - char as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| line.split(" ").collect::<Vec<&str>>()).map(|val| {
        let hand = char_offset(val.get(1).unwrap(), 'X');
        let other = char_offset(val.get(0).unwrap(), 'A');

        result(hand, other) + hand + 1
    }).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| line.split(" ").collect::<Vec<&str>>()).map(|val| {
        let win_or_lose = char_offset(val.get(1).unwrap(), 'X');
        let other = char_offset(val.get(0).unwrap(), 'A');

        let hand = BEATS[((other+win_or_lose) % BEATS.len() as u32) as usize];

        result(hand, other) + hand + 1
    }).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
