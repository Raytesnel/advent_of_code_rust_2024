pub fn assigment_6_a(input_lines: &str)->i32{0}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    #[test]
    fn test_assigment_integration_5_a() {
        let file_contents = fs::read_to_string("../test_input/assignment_6.txt").expect("couldn't read file");
        let expected_value = 143;

        assert_eq!(assigment_6_a(&file_contents), expected_value)
    }
}