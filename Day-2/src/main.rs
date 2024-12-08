use std::fs;

fn main() {
    println!("Safe report count is {:?}", calculate_safe_reports_count());
    println!(
        "Safe report count (after fixing) is {:?}",
        calculate_safe_reports_count_after_fixing()
    );
}

fn calculate_safe_reports_count() -> usize {
    let file_content = fs::read_to_string("test-input.txt").expect("Test File should be specified");
    let safe_reports_count = file_content
        .lines()
        .filter(|line| {
            let report_items: Vec<i32> = line
                .split_whitespace()
                .filter_map(|item| item.parse::<i32>().ok())
                .collect();
            let mut is_report_valid = true;
            let mut is_value_increasing_trend = true;
            let mut previous_report_item = 0;
            for (index, report_item) in report_items.iter().enumerate() {
                if index > 0 {
                    if *report_item == previous_report_item
                        || (*report_item > previous_report_item
                            && *report_item - previous_report_item > 3)
                        || (*report_item < previous_report_item
                            && previous_report_item - *report_item > 3)
                    {
                        is_report_valid = false;
                        break;
                    }

                    let is_value_increasing = *report_item > previous_report_item;
                    if index == 1 {
                        is_value_increasing_trend = is_value_increasing;
                    } else if index > 1 && is_value_increasing != is_value_increasing_trend {
                        is_report_valid = false;
                        break;
                    }
                }

                previous_report_item = *report_item;
            }

            is_report_valid
        })
        .count();
    safe_reports_count
}

fn calculate_safe_reports_count_after_fixing() -> usize {
    let file_content = fs::read_to_string("test-input.txt").expect("Test File should be specified");
    let safe_reports_count = file_content
        .lines()
        .filter(|line| {
            let report_items: Vec<i32> = line
                .split_whitespace()
                .filter_map(|item| item.parse::<i32>().ok())
                .collect();
            let mut is_report_valid = true;
            let mut is_value_increasing_trend = true;
            let mut previous_report_item = 0;
            let mut failed_attempts = 0;
            for (index, report_item) in report_items.iter().enumerate() {
                if index > 0 {
                    if *report_item == previous_report_item
                        || (*report_item > previous_report_item
                            && *report_item - previous_report_item > 3)
                        || (*report_item < previous_report_item
                            && previous_report_item - *report_item > 3)
                    {
                        if failed_attempts == 0 {
                            failed_attempts = 1;
                            continue;
                        } else {
                            is_report_valid = false;
                            break;
                        }
                    }

                    let is_value_increasing = *report_item > previous_report_item;
                    if index == 1 {
                        is_value_increasing_trend = is_value_increasing;
                    } else if index > 1 && is_value_increasing != is_value_increasing_trend {
                        if failed_attempts == 0 {
                            failed_attempts = 1;
                        } else {
                            is_report_valid = false;
                            break;
                        }
                    }
                }

                previous_report_item = *report_item;
            }

            is_report_valid
        })
        .count();
    safe_reports_count
}
