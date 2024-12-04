advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let h = input.lines().fold(0, |acc, s| {
        acc + s.matches("XMAS").count() + s.matches("SAMX").count()
    }) as u32;
    let n = input.lines().next().unwrap().len();
    let vd = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>()
        .windows(4)
        .fold(0, |acc, a| {
            let st = (0..n).fold([String::new(), String::new(), String::new()], |s, i| {
                let v = s[0].clone() + &(0..4).fold(String::new(), |r, j| r + &a[j][i].to_string()) + &" ";
                let d = if i > n - 4 { s[1].clone() } else { s[1].clone() + &(0..4).fold(String::new(), |r, j| r + &a[j][i+j].to_string()) + &" " };
                let g = if i < 3 { s[2].clone() } else { s[2].clone() + &(0..4).fold(String::new(), |r, j| r + &a[j][i-j].to_string()) + &" " };
                [v, d, g]
            });
            acc + (0..=2).fold(0, |ax, k| ax + st[k].matches("XMAS").count() + st[k].matches("SAMX").count())
        }) as u32;
    Some(h + vd)
}

pub fn part_two(input: &str) -> Option<u32> {
    let m: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut s = 0;
    for (x, vx) in m.iter().enumerate() {
        if x == 0 || x == m.len() - 1 { continue; }
        else {
            for (y, c) in vx.iter().enumerate() {
                if y == 0 || y == vx.len() - 1 || c != &'A' { continue; }
                else {
                    if ((m[x-1][y-1] == 'M' && m[x+1][y+1] == 'S')
                        || (m[x-1][y-1] == 'S' && m[x+1][y+1] == 'M'))
                        && ((m[x-1][y+1] == 'M' && m[x+1][y-1] == 'S')
                        || (m[x-1][y+1] == 'S' && m[x+1][y-1] == 'M')) {
                            s += 1;
                    }
                }
            }
        }
    }
    Some (s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
