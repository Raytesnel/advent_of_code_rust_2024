pub fn assigment_6_a(input_lines: &str) -> i32 {
    println!("{}", input_lines);
    0
}
struct Board{
    obstacles_cor: Vec<Vec<usize>>,
    player_cor:Vec<usize>,
    board: Vec<usize>,
}

fn mapping(input_lines: &str) -> Board {
    let height = input_lines.lines().count();
    let length = input_lines
        .lines()
        .next()
        .map(|line| line.len())
        .unwrap_or(0);

    let mut obstacles: Vec<Vec<usize>> = Vec::new();
    let mut arrow = Vec::<usize>::new();

    println!("height: {:?}, length: {:?}", height, length);
    for (y, line) in input_lines.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {obstacles.push(vec![y, x]);}
                '>' | '^' | '<' | 'v' => {arrow = vec![y, x];}
                _ => {}
            }
        }
    }

Board{obstacles_cor:obstacles,player_cor:arrow,board:vec![height,length]}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_assigment_integration_5_a() {
        let file_contents =
            fs::read_to_string("./test_input/assignment_6.txt").expect("couldn't read file");
        let expected_value = 143;

        assert_eq!(assigment_6_a(&file_contents), expected_value)
    }
    #[test]
    fn test_mapping() {
        let file_contents =
            fs::read_to_string("./test_input/assignment_6.txt").expect("couldn't read file");
        let result = mapping(&file_contents);
        println!("mapping:\nobstacles:{:?}\nplayer:{:?}\nboard:{:?}", result.obstacles_cor, result.player_cor,result.board);
    }
}
// pinpoint all obstacles. and location          done
// find out what direction the arrow point
// check next obstacle and move location
// add all moved steps to the counter
