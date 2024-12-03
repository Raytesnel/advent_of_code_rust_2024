
fn check_if_valid(chambers: &Vec<i32>) -> bool {
    let mut chamber_iter = chambers.iter().peekable();
    let mut directions = Vec::new();
    while let Some(&current) = chamber_iter.next() {
        if let Some(&next) = chamber_iter.peek() {
            let step = next - current;
            if step <= 3 && step > 0 {
                directions.push(true);
            } else if step < 0 && step >= -3 { directions.push(false); } else { return false; }
        }
    }
    directions.iter().all(|&x| x == directions[0])
}

fn count_correct_layers(file_contents: &str)->i32{
    let mut counter = 0 ;
    for layer in file_contents.lines(){
        let chambers: Vec<i32> = layer
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if check_if_valid(&chambers){
            counter += 1;
        }
    }
    counter
}

pub fn assigment_2_a(file_contents: &str) -> i32 {
    count_correct_layers(&file_contents)
}

#[cfg(test)]
mod tests {
    use std::fs;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_code_is_safe_with_decrease() {
        let list_chambers = vec![7, 6, 4, 2, 1];
        let expected_check = true;
        assert_eq!(check_if_valid(&list_chambers), expected_check)
    }

    #[test]
    fn test_code_is_not_safe_to_big_increase() {
        let list_chambers = vec![1, 2, 7, 8, 9];
        let expected_check = false;
        assert_eq!(check_if_valid(&list_chambers), expected_check)
    }

    #[test]
    fn test_code_is_not_safe_to_big_decrease() {
        let list_chambers = vec![9, 7, 6, 2, 1];
        let expected_check = false;
        assert_eq!(check_if_valid(&list_chambers), expected_check)
    }

    #[test]
    fn test_code_is_not_safe_decrease_and_increase() {
        let list_chambers = vec![1, 3, 2, 4, 5];
        let expected_check = false;
        assert_eq!(check_if_valid(&list_chambers), expected_check)
    }

    #[test]
    fn test_code_is_not_safe_has_no_increase_or_decrease() {
        let list_chambers = vec![8, 6, 4, 4, 1];
        let expected_check = false;
        assert_eq!(check_if_valid(&list_chambers), expected_check)
    }

    #[test]
    fn test_code_is_safe_increase() {
        let list_chambers = vec![1, 3, 6, 7, 9];
        let expected_check = true;
        assert_eq!(check_if_valid(&list_chambers), expected_check)
    }

    #[test]
    fn test_count_correct_lines() {
        let numbers = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let total_count = 2;
        assert_eq!(count_correct_layers(&numbers),total_count)
    }

    #[test]
    fn test_e_2_e_day2_a(){
        let file_contents = fs::read_to_string("input/assigment_2.txt")
            .expect("LogRocket: Should have been able to read the file{}");
        let total_count = 407;
        assert_eq!(assigment_2_a(&file_contents),total_count)
    }

}
