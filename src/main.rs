use std::fs;

mod day_1;
mod day_2;

fn main() {
    use std::time::Instant;
    let day_1_contents = read_file("input/assigment_1.txt");
    let day_2_contests = read_file("input/assigment_2.txt");
    let now = Instant::now();
    println!("day_1_a answer is: {:?}", day_1::assigment_1_a(&day_1_contents));
    println!("day_1_b answer is: {:?}", day_1::assigment_1_b(&day_1_contents));
    println!("day_2_a answer is: {:?}", day_2::assigment_2_a(&day_2_contests));
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
}


fn read_file(input_path: &str) -> String {
    //read the file returns a really long string
    let file_contents = fs::read_to_string(input_path)
        .expect("LogRocket: Should have been able to read the file{}");
    return file_contents;
}