use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Regex::new(r"mul\((\d+),(\d+)\)")
            .unwrap()
            .captures_iter(input)
            .map(|c| c.extract().1)
            .fold(0, |acc, n: [&str; 2]| {
                acc + (n[0].parse::<u32>().unwrap() * n[1].parse::<u32>().unwrap())
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let mut s = 0;
    let pat = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)()()|don't\(\)()()").unwrap();
    for c in pat.captures_iter(input).map(|c| c.extract::<2>()) {
        if c.0 == "do()" { enabled = true; }
        else if c.0 == "don't()" { enabled = false; }
        else if !enabled { continue; }
        else {
            let n = c.1;
            s += n[0].parse::<u32>().unwrap() * n[1].parse::<u32>().unwrap()
        }
    }
    Some(s)
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
