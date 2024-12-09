static DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),   // Right
    (0, -1),  // Left
    (1, 0),   // Down
    (-1, 0),  // Up
    (1, 1),   // Diagonal down-right
    (1, -1),  // Diagonal down-left
    (-1, 1),  // Diagonal up-right
    (-1, -1), // Diagonal up-left
];
type DirectionMas = ((isize, isize), (isize, isize), (isize, isize));
static DIRECTIONS_MAS: [DirectionMas; 4] = [
    ((1, 1),(0,0),(-1,-1)),   // Diagonal down-right->up-left
    ((1, -1),(0,0),(-1,1)),  // Diagonal down-left->up-right
    ((-1, 1),(0,0),(1,-1)),  // Diagonal up-right->down-left
    ((-1, -1),(0,0),(1,1)), // Diagonal up-left-> down-right
];

static TARGET: &[char] = &['X', 'M', 'A', 'S'];
static TARGET_MAS: &[char] = &['M', 'A', 'S'];
#[derive(Debug, PartialEq)]
pub struct CoordinateX {
    x_pos: i32,
    y_pos: i32,
}

fn check_xmas_in_block(block: &str, cordinate_x: &CoordinateX) -> usize {
    let lines: Vec<&str> = block.trim().lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut count = 0;

    for &(dx, dy) in DIRECTIONS.iter() {
        let mut found = true;
        for i in 0..TARGET.len() {
            let x = cordinate_x.x_pos as isize + i as isize * dx;
            let y = cordinate_x.y_pos as isize + i as isize * dy;
            if x < 0
                || x >= rows as isize
                || y < 0
                || y >= cols as isize
                || lines[y as usize].chars().nth(x as usize).unwrap() != TARGET[i]
            {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
        }
    }

    count
}
fn check_mas_in_block(block: &str, cordinate_x: &CoordinateX) -> usize {
    let lines: Vec<&str> = block.trim().lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut match_count = 0;

    for &(dir1, dir2, dir3) in DIRECTIONS_MAS.iter() {
        let mut found = true;
        let directions = [dir1, dir2, dir3];
        for (count, &(dx, dy)) in directions.iter().enumerate() {
            let x = cordinate_x.x_pos as isize + dx;
            let y = cordinate_x.y_pos as isize + dy;

            if x < 0
                || x >= cols as isize
                || y < 0
                || y >= rows as isize
                || lines[y as usize].chars().nth(x as usize).unwrap() != TARGET_MAS[count]
            {
                found = false;
                break;
            }
        }
        if found {
            match_count += 1;
            if match_count >= 2 {
                return 1; // Return early if at least two matches are found
            }
        }
    }

    0 // Return 0 if fewer than 2 occurrences are found
}
fn found_char_in_block<F>(block: &str, char_to_find: &str, func_: F) -> usize
where
    F: Fn(&str, &CoordinateX) -> usize,
{
    let mut counting = 0;
    for (line_number, line) in block.trim().lines().enumerate() {
        let position = line.match_indices(char_to_find).collect::<Vec<(usize, &str)>>();
        for pos in position {
            counting += func_(
                &block,
                &CoordinateX {
                    x_pos: pos.0 as i32,
                    y_pos: line_number as i32,
                },
            );
        }
    }
    counting
}
pub fn assigment_4_a(file_contents: &str)->i32{
    found_char_in_block(&file_contents, "X", check_xmas_in_block) as i32
}
pub fn assigment_4_b(file_contents: &str)->i32{
    found_char_in_block(&file_contents, "A", check_mas_in_block) as i32
}


#[cfg(test)]
mod tests {
    use std::fs;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_xmas_found() {
        let block = "
.........
.SAMSMAS.
.SAMAMAS.
.SAMMMAS.
.SAMXMAS.
.SAMMMAS.
.SAMAMAS.
.SAMSMAS.
.........
";
        assert_eq!(
            check_xmas_in_block(block, &CoordinateX { x_pos: 4, y_pos: 4 }),
            8
        );
    }

    #[test]
    fn test_xmas_found_half() {
        let block = "
SAMSMAP
SAMAMAS
SAMMMAS
SLMXMAS
SAMZMAS
SAMAMAS
SAMSMAS
";
        assert_eq!(
            check_xmas_in_block(block, &CoordinateX { x_pos: 3, y_pos: 3 }),
            5
        );
    }

    #[test]
    fn test_xmas_found_dots() {
        let block = "
.......
SAMAMAS
SAMMMAS
SLMXMAS
SAMZMAS
SAMAMAS
SAMSMAS
";
        assert_eq!(
            check_xmas_in_block(block, &CoordinateX { x_pos: 3, y_pos: 3 }),
            3
        );
    }
    #[test]
    fn test_xmas_nothin_found() {
        let block = "
.......
.......
.......
.......
.......
.......
.......
";
        assert_eq!(
            check_xmas_in_block(block, &CoordinateX { x_pos: 3, y_pos: 3 }),
            0
        );
    }

    #[test]
    fn test_xmas_one_found() {
        let block = "
1234567
SAMXabc
defghij
00.....
11.....
22......
33......
";
        assert_eq!(
            check_xmas_in_block(block, &CoordinateX { x_pos: 3, y_pos: 1 }),
            1
        );
    }
    #[test]
    fn test_example() {
        let block = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let founds = 18;

        assert_eq!(found_char_in_block(block, "X", check_xmas_in_block), founds);
    }

    #[test]
    fn test_example_b() {
        let block = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let founds = 9;

        assert_eq!(found_char_in_block(block, "A", check_mas_in_block), founds);
    }
    #[test]
    fn test_e2e_4_a(){
        let file_contents = fs::read_to_string("input/assigment_4.txt")
            .expect("LogRocket: Should have been able to read the file{}");
        let founds = 2507;
        assert_eq!(found_char_in_block(&file_contents, "X", check_xmas_in_block), founds);
    }
    #[test]
    fn test_e2e_4_b(){
        let file_contents = fs::read_to_string("input/assigment_4.txt")
            .expect("LogRocket: Should have been able to read the file{}");
        let founds = 1969;
        assert_eq!(found_char_in_block(&file_contents, "A", check_mas_in_block), founds);
    }
}
