use regex::Regex;

fn extract_mul_matches<F>(input: &str) -> Vec<String>
{
    let re = Regex::new(r"mul\(\d*,\d*\)|don't\(\)|do\(\)").unwrap();
    re.find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

fn parse_mul_capture_with_check(item_list: Vec<String>) -> Vec<i32> {
    let mut do_enabled = true;
    let mut results = Vec::new();

    for item in item_list {
        match item.as_str() {
            "don't()" => {
                do_enabled = false;
            }
            "do()" => {
                do_enabled = true;
            }
            _ if do_enabled && item.starts_with("mul(") => {
                let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
                if let Some(cap) = re.captures(&item) {
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    results.push(x * y);
                }
            }
            _ => { /* println!(" a {:?} while do is in {:?}", item, do_enabled)*/ }
        }
    }

    results
}

fn parse_mul_capture_default(item_list: Vec<String>) -> Vec<i32> {
    let mut results = Vec::new();

    for item in item_list {
        if item.as_str().starts_with("mul(") {
            let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
            if let Some(cap) = re.captures(&item) {
                let x: i32 = cap[1].parse().unwrap();
                let y: i32 = cap[2].parse().unwrap();
                results.push(x * y);
            }
        }
    }

    results
}

pub fn assigment_3_b(file_contents: &str) -> i32 {
    let combined_input = file_contents.replace('\n', " ");
    let matches = extract_mul_matches::<Vec<String>>(&combined_input);
    parse_mul_capture_with_check(matches).iter().sum()
}

pub fn assigment_3_a(file_contents: &str) -> i32 {
    file_contents
        .lines()
        .flat_map(|l| parse_mul_capture_default(extract_mul_matches::<Vec<String>>(l)))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_all_hits() {
        let stringy = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let list_found_sets = vec![(2 * 4), (5 * 5), (11 * 8), (8 * 5)];
        assert_eq!(parse_mul_capture_default(extract_mul_matches::<Vec<String>>(&stringy)), list_found_sets)
    }

    #[test]
    fn test_find_all_hits_dont() {
        let stringy = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let list_found_sets = vec![(2 * 4 ),( 8 * 5)];
        assert_eq!(parse_mul_capture_with_check(extract_mul_matches::<Vec<String>>(&stringy)), list_found_sets)
    }

    #[test]
    fn test_3_a() {
        let stringy = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected_number = 161;
        assert_eq!(assigment_3_a(&stringy), expected_number)
    }
    #[test]
    fn test_3_b() {
        let stringy = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected_number = 48;
        assert_eq!(assigment_3_b(&stringy), expected_number)
    }

    #[test]
    fn test_e_2_e_day3_a() {
        let file_contents = fs::read_to_string("input/assigment_3.txt")
            .expect("LogRocket: Should have been able to read the file{}");
        let total_count = 164730528;
        assert_eq!(assigment_3_a(&file_contents), total_count)
    }

    #[test]
    fn test_e_2_e_day3_b() {
        let file_contents = fs::read_to_string("input/assigment_3.txt")
            .expect("LogRocket: Should have been able to read the file{}");
        let total_count = 70478672;
        assert_eq!(assigment_3_b(&file_contents), total_count)
    }
}
