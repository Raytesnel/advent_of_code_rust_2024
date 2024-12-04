use regex::{Captures, Regex};

fn extract_mul_matches<F>(input: &str, parse: F) -> Vec<i32>
where
    F: Fn(&Captures) -> i32,
{
    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    re.captures_iter(input).map(|cap| parse(&cap)).collect()
}

fn parse_mul_capture_default(cap: &Captures) -> i32 {
    let x: i32 = cap[1].parse().unwrap();
    let y: i32 = cap[2].parse().unwrap();
    x* y
}

pub fn assigment_3_a(file_contents: &str) -> i32 {
    file_contents
        .lines()
        .flat_map(|l| extract_mul_matches(l,parse_mul_capture_default))
        .sum()}

#[cfg(test)]
mod tests {
    use std::fs;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_all_hits() {
        let stringy = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let list_found_sets = vec![(2* 4), (5* 5), (11* 8), (8* 5)];
        assert_eq!(extract_mul_matches(&stringy,parse_mul_capture_default), list_found_sets)
    }

    #[test]
    fn test_3_a() {
        let stringy = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected_number = 161;
        assert_eq!(assigment_3_a(&stringy), expected_number)
    }

    #[test]
    fn test_e_2_e_day3_a() {
        let file_contents = fs::read_to_string("input/assigment_3.txt")
            .expect("LogRocket: Should have been able to read the file{}");
        let total_count = 164730528;
        assert_eq!(assigment_3_a(&file_contents), total_count)
    }
}
