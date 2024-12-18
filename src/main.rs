use std::fs;
use std::time::Instant;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn main() {
    let inputs = [
        (
            "input/assignment_1.txt",
            vec![day_1::assigment_1_a, day_1::assigment_1_b],
        ),
        (
            "input/assignment_2.txt",
            vec![day_2::assigment_2_a, day_2::assigment_2_b],
        ),
        (
            "input/assignment_3.txt",
            vec![day_3::assigment_3_a, day_3::assigment_3_b],
        ),
        (
            "input/assignment_4.txt",
            vec![day_4::assigment_4_a, day_4::assigment_4_b],
        ),
        (
            "input/assignment_5.txt",
            vec![day_5::assigment_5_a, day_5::assigment_5_b],
        ),
    ];

    for (file_path, assignments) in inputs {
        let contents = read_file(file_path);

        for (i, assignment) in assignments.iter().enumerate() {
            let now = Instant::now();
            let result = assignment(&contents);
            let elapsed = now.elapsed();
            println!(
                "Result for {:<25} part-{:?}: {:<15} (Time elapsed: {:>8.2?}µs)",
                file_path,
                i + 1,
                result,
                elapsed.as_micros()
            );
        }
    }
}

fn read_file(input_path: &str) -> String {
    fs::read_to_string(input_path).expect("LogRocket: Should have been able to read the file{}")
}
