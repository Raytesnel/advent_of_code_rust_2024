#[derive(Debug, PartialEq)]
pub struct ListsInput {
    right_list: Vec<i32>,
    left_list: Vec<i32>,
}

pub fn assigment_1_a(file_contents: String) -> i32 {
    compute_differences(&split_input_into_lists(&file_contents))
}
pub fn assigment_1_b(file_contents: String) -> i32 {
    count_occurrences(&split_input_into_lists(&file_contents))
}

fn compute_differences(lists: &ListsInput) -> i32 {
    let mut total_difference = 0;
    let mut left_iter = lists.left_list.iter();
    let mut right_iter = lists.right_list.iter();

    while let (Some(&left), Some(&right)) = (left_iter.next(), right_iter.next()) {
        total_difference += (left - right).abs()
    }
    total_difference
}

fn count_occurrences(lists: &ListsInput) -> i32 {
    lists.left_list.iter().map(|&left_item| {
        lists.right_list.iter().filter(|&&right_item| right_item == left_item).count() as i32 * left_item
    }).sum()
}

fn split_input_into_lists(file_contents: &str) -> ListsInput {
    let mut left_items: Vec<i32> = vec![];
    let mut right_items: Vec<i32> = vec![];

    for line in file_contents.lines() {
        let mut items = line.split_whitespace();
        let left = items.next().unwrap().parse::<i32>().unwrap();
        let right = items.next().unwrap().parse::<i32>().unwrap();
        left_items.push(left);
        right_items.push(right);
    }
    left_items.sort();
    right_items.sort();
    ListsInput { right_list: right_items, left_list: left_items }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_lists() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3".to_string();
        let left_items = vec![1, 2, 3, 3, 3, 4];
        let right_items = vec![3, 3, 3, 4, 5, 9];
        let expected_lists = ListsInput { right_list: right_items, left_list: left_items };
        assert_eq!(split_input_into_lists(&test_input), expected_lists);
    }

    #[test]
    fn test_find_occurences(){
        let left_items = vec![1, 2, 3, 3, 3, 4];
        let right_items = vec![3, 3, 3, 4, 5, 9];
        let input_lists = ListsInput { right_list: right_items, left_list: left_items };
        let expected_total = 31;
        assert_eq!(count_occurrences(&input_lists), expected_total)

    }

    #[test]
    fn test_get_total_number() {
        let left_items = vec![1, 2, 3, 3, 3, 4];
        let right_items = vec![3, 3, 3, 4, 5, 9];
        let input_lists = ListsInput { right_list: right_items, left_list: left_items };
        let expected_total = 11;
        assert_eq!(compute_differences(&input_lists), expected_total)
    }

    #[test]
    fn test_integration() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3".to_string();
        assert_eq!(assigment_1_a(test_input), 11);
    }
}