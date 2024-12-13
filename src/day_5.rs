#[derive(Debug, PartialEq)]
pub struct ListsInput {
    rules: Vec<Vec<usize>>,
    item_to_check: Vec<Vec<usize>>,
}

fn parse(input: &str) -> ListsInput {
    let mut rules = vec![Vec::new(); 100];
    let mut updates = Vec::new();

    let mut line_iter = input.lines().into_iter();

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        let mut numbers: Vec<_> = line.split('|')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let x: usize = numbers[0];
        let y: usize = numbers[1];
        rules[x].push(y);
    }
    while let Some(line) = line_iter.next() {
        let numbers = line.split(",").map(|number| number.parse::<usize>()).collect::<Result<Vec<usize>, _>>().unwrap();
        updates.push(numbers);
    }
    ListsInput{ rules:rules, item_to_check:updates}
}

fn give_midle_number_if_correct(update: &Vec<usize>, rules: &Vec<Vec<usize>>) -> Option<usize> {
    for i in 0..update.len() - 1 {
        if !rules[update[i]].contains(&update[i + 1]) {
            return None;
        }
    }
    let mid = (update.len() - 1) / 2;
    Some(update[mid])
}
pub fn assigment_5_a(_input: &str) -> usize {
    let parsed_input = parse(_input);
    parsed_input.item_to_check
        .iter()
        .filter_map(|update| give_midle_number_if_correct(update, &parsed_input.rules))
        .sum::<usize>()
        .into()
}

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

fn find_next_page(update: &Vec<usize>, rules: &Vec<Vec<usize>>) -> Option<usize> {
    for i in 0..update.len() {
        let rule = &rules[update[i]];
        let mut found = true;
        for n in 0..update.len() {
            if n == i {
                continue;
            }

            if rule.contains(&update[n]) {
                found = false;
                break;
            }
        }
        if found {
            return Some(i);
        }
    }
    None
}


fn check_2(update: &mut Vec<usize>, rules: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;

    for _ in 0..=update.len() / 2 {
        if let Some(page_index) = find_next_page(update, rules) {
            result = update.remove(page_index);
        }
    }
    result
}

pub fn assigment_5_b(_input: &str) -> usize {
    let mut inputs = parse(_input);
    inputs.item_to_check
        .iter_mut()
        .filter(|update| give_midle_number_if_correct(update, &inputs.rules).is_none())
        .map(|mut update| check_2(&mut update, &inputs.rules))
        .sum::<usize>()
        .into()
}

// #[cfg(test)]
// mod part_2_tests {

//     #[test_case(include_str!("_test.txt"), 123)]
//     fn example_input(input: &str, expected: usize) {
//         assert_eq!(part_2(input), expected.into());
//     }
//
//     #[test_case(4598)]
//     fn real_input(expected: usize) {
//         assert_eq!(part_2(_INPUT), expected.into());
//     }
// }
