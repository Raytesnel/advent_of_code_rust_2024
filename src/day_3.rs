use regex::Regex;

fn find_mul_expressions(input: &str) -> Vec<i32> {
    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    let mut results = Vec::new();

    for cap in re.captures_iter(input) {
        let x:i32 = cap[1].parse().unwrap();
        let y:i32 = cap[2].parse().unwrap();
        results.push(x*y);
    }
    results
}
pub fn assigment_3_a(file_contents: &str) -> i32 {
    file_contents
        .lines()
        .flat_map(|l| find_mul_expressions(l))
        .sum()}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_all_hits() {
        let stringy = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let list_found_sets = vec![(2* 4), (5* 5), (11* 8), (8* 5)];
        assert_eq!(find_mul_expressions(&stringy), list_found_sets)
    }

    #[test]
    fn test_3_a(){
        let stringy = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected_number = 161;
        assert_eq!(assigment_3_a(&stringy),expected_number)
    }
}
