use std::fs;

mod day_1;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let day_1_contents = read_file("input/assigment_1.txt");
    println!("day_1_a answer is: {:?}",day_1::assigment_1_a(&day_1_contents));
    println!("day_1_b answer is: {:?}",day_1::assigment_1_b(&day_1_contents));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}


fn read_file(input_path: &str) -> String {
    //read the file returns a really long string
    let file_contents = fs::read_to_string(input_path)
        .expect("LogRocket: Should have been able to read the file{}");
    return file_contents;
}