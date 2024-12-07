use std::fs;

fn main() {
    println!("Distance between elements: {:?}", calculate_distance());
    println!(
        "Similarity Score between elements: {:?}",
        calculate_similarity_score()
    );
}

// Part 1
fn calculate_distance() -> i32 {
    let file_content = fs::read_to_string("test-input.txt").expect("Test File should be specified");
    let mut left_numbers: Vec<i32> = vec![];
    let mut right_numbers: Vec<i32> = vec![];
    file_content.lines().for_each(|line| {
        let mut numbers_string = line.split_whitespace();
        let first_number = numbers_string.next();
        let second_number = numbers_string.last();
        if let (Some(first_number), Some(second_number)) = (first_number, second_number) {
            let first_number_value = first_number
                .parse::<i32>()
                .expect("First Number should be parsable");
            let second_number_value = second_number
                .parse::<i32>()
                .expect("Second Number should be parsable");
            left_numbers.push(first_number_value);
            right_numbers.push(second_number_value);
        }
    });

    left_numbers.sort();
    right_numbers.sort();
    let total_distance: i32 = file_content
        .lines()
        .enumerate()
        .map(|(index, _)| {
            let left_lowest_number = left_numbers[index];
            let right_lowest_number = right_numbers[index];
            if left_lowest_number >= right_lowest_number {
                left_lowest_number - right_lowest_number
            } else {
                right_lowest_number - left_lowest_number
            }
        })
        .sum();
    total_distance
}

// Part 2
fn calculate_similarity_score() -> i32 {
    let file_content = fs::read_to_string("test-input.txt").expect("Test File should be specified");
    let mut left_numbers: Vec<i32> = vec![];
    let mut right_numbers: Vec<i32> = vec![];
    file_content.lines().for_each(|line| {
        let mut numbers_string = line.split_whitespace();
        let first_number = numbers_string.next();
        let second_number = numbers_string.last();
        if let (Some(first_number), Some(second_number)) = (first_number, second_number) {
            let first_number_value = first_number
                .parse::<i32>()
                .expect("First Number should be parsable");
            let second_number_value = second_number
                .parse::<i32>()
                .expect("Second Number should be parsable");
            left_numbers.push(first_number_value);
            right_numbers.push(second_number_value);
        }
    });

    let mut total_similarity_score = 0;
    for left_number in left_numbers {
        let similarity_score = right_numbers
            .iter()
            .filter(|right_number| left_number == **right_number)
            .count();
        total_similarity_score += left_number * (similarity_score as i32);
    }

    total_similarity_score
}
