use crate::day_5::assigment_5_a;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_assigment_integration_5_a() {
        let list_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13\r\n\r\n75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let expected_value = 143;

        assert_eq!(assigment_5_a(&list_input), expected_value)
    }
}