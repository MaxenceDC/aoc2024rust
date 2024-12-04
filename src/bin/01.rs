advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut row1 = Vec::with_capacity(1000);
    let mut row2 = Vec::with_capacity(1000);
    input.lines().for_each(|l| {
        let mut nums = l.split("   ").map(|n| n.parse::<u32>().unwrap());
        row1.push(nums.next().unwrap());
        row2.push(nums.next().unwrap());
    });
    (row1, row2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut row1, mut row2) = parse_input(input);
    row1.sort();
    row2.sort();
    Some(row1.iter().zip(row2).map(|(n1, n2)| n1.abs_diff(n2)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (row1, row2) = parse_input(input);
    Some(
        row1.iter()
            .map(|n| n * row2.iter().filter(|m| m == &n).count() as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
