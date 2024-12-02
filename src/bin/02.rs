advent_of_code::solution!(2);

fn is_safe(v: &Vec<u32>) -> bool {
    v.is_sorted_by(|a, b| a < b && b - a <= 3) || v.is_sorted_by(|a, b| a > b && a - b <= 3)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|s|
                is_safe(
                    &s
                        .split_ascii_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect(),
                )
            )
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|s| {
                let v: Vec<u32> = s.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
                if is_safe(&v) { return true }
                else {
                    for i in 0..v.len() {
                        let mut new_v = v.clone();
                        new_v.remove(i);
                        if is_safe(&new_v) { return true }
                    }
                    return false
                }
            })
            .count() as u32
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
