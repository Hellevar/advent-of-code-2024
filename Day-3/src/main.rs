use std::fs;

fn main() {
    println!("Part 1 - answer is {:?}", do_multiplications());
    println!(
        "Part 2 - answer is {:?}",
        do_multiplications_on_enabled_instructions()
    );
}

// Part 1
fn do_multiplications() -> i32 {
    let file_content = fs::read_to_string("test-input.txt").expect("Test File should be specified");
    let mut total_product = 0;
    for line in file_content.lines() {
        let mut first_number = String::from("");
        let mut second_number = String::from("");
        let mut m_encountered = false;
        let mut u_encountered = false;
        let mut l_encountered = false;
        let mut comma_encountered = false;
        let mut first_brace_encountered = false;
        let mut second_brace_encountered = false;
        for character in line.chars() {
            if !m_encountered {
                if character == 'm' {
                    m_encountered = true;
                    continue;
                } else {
                    continue;
                }
            }

            if !u_encountered {
                if character == 'u' {
                    u_encountered = true;
                    continue;
                } else {
                    m_encountered = false;
                    continue;
                }
            }

            if !l_encountered {
                if character == 'l' {
                    l_encountered = true;
                    continue;
                } else {
                    m_encountered = false;
                    u_encountered = false;
                    continue;
                }
            }

            if !first_brace_encountered {
                if character == '(' {
                    first_brace_encountered = true;
                    continue;
                } else {
                    m_encountered = false;
                    u_encountered = false;
                    l_encountered = false;
                    continue;
                }
            }

            if !comma_encountered {
                if character.is_numeric() {
                    first_number.push(character);
                    continue;
                } else if character == ',' && !first_number.is_empty() {
                    comma_encountered = true;
                    continue;
                } else {
                    m_encountered = false;
                    u_encountered = false;
                    l_encountered = false;
                    first_brace_encountered = false;
                    first_number = String::from("");
                    continue;
                }
            }

            if !second_brace_encountered {
                if character.is_numeric() {
                    second_number.push(character);
                    continue;
                } else if character == ')' && !second_number.is_empty() {
                    total_product += first_number.parse::<i32>().unwrap()
                        * second_number.parse::<i32>().unwrap();
                    m_encountered = false;
                    u_encountered = false;
                    l_encountered = false;
                    first_brace_encountered = false;
                    first_number = String::from("");
                    comma_encountered = false;
                    second_number = String::from("");
                    second_brace_encountered = false;
                    continue;
                } else {
                    m_encountered = false;
                    u_encountered = false;
                    l_encountered = false;
                    first_brace_encountered = false;
                    first_number = String::from("");
                    comma_encountered = false;
                    second_number = String::from("");
                    continue;
                }
            }
        }
    }
    total_product
}

// Part 2
fn do_multiplications_on_enabled_instructions() -> i32 {
    let file_content = fs::read_to_string("test-input.txt").expect("Test File should be specified");
    let mut total_product = 0;
    let mut add_enabled = true;
    for line in file_content.lines() {
        let mut accumulator = String::from("");
        let mut first_number = String::from("");
        let mut second_number = String::from("");
        let mut comma_encountered = false;
        let mut first_brace_encountered = false;
        let mut second_brace_encountered = false;
        for character in line.chars() {
            if accumulator.is_empty() {
                if character == 'd' || character == 'm' {
                    accumulator.push(character);
                    continue;
                } else {
                    continue;
                }
            }

            if accumulator == "d" {
                if character == 'o' {
                    accumulator.push(character);
                    continue;
                } else {
                    accumulator = String::from("");
                    continue;
                }
            }

            if accumulator == "do" {
                if character == '(' || character == 'n' {
                    accumulator.push(character);
                    continue;
                } else {
                    accumulator = String::from("");
                    continue;
                }
            }

            if accumulator == "don" {
                if character == '\'' {
                    accumulator.push(character);
                    continue;
                } else {
                    accumulator = String::from("");
                    continue;
                }
            }

            if accumulator == "don'" {
                if character == 't' {
                    accumulator.push(character);
                    continue;
                } else {
                    accumulator = String::from("");
                    continue;
                }
            }

            if accumulator == "don't" {
                if character == '(' {
                    accumulator.push(character);
                    continue;
                } else {
                    accumulator = String::from("");
                    continue;
                }
            }

            if accumulator == "don't(" || accumulator == "do(" {
                if character == ')' {
                    add_enabled = accumulator == "do(";
                }

                accumulator = String::from("");
                continue;
            }

            if accumulator == "m" {
                if character == 'u' {
                    accumulator.push(character);
                    continue;
                } else {
                    accumulator = String::from("");
                    continue;
                }
            }

            if accumulator == "mu" {
                if character == 'l' {
                    accumulator.push(character);
                    continue;
                } else {
                    accumulator = String::from("");
                    continue;
                }
            }

            if accumulator == "mul" {
                if !first_brace_encountered {
                    if character == '(' {
                        first_brace_encountered = true;
                        continue;
                    } else {
                        accumulator = String::from("");
                        continue;
                    }
                }

                if !comma_encountered {
                    if character.is_numeric() {
                        first_number.push(character);
                        continue;
                    } else if character == ',' && !first_number.is_empty() {
                        comma_encountered = true;
                        continue;
                    } else {
                        accumulator = String::from("");
                        first_brace_encountered = false;
                        first_number = String::from("");
                        continue;
                    }
                }

                if !second_brace_encountered {
                    if character.is_numeric() {
                        second_number.push(character);
                        continue;
                    } else if character == ')' && !second_number.is_empty() {
                        if add_enabled {
                            total_product += first_number.parse::<i32>().unwrap()
                                * second_number.parse::<i32>().unwrap();
                        }

                        accumulator = String::from("");
                        first_brace_encountered = false;
                        first_number = String::from("");
                        comma_encountered = false;
                        second_number = String::from("");
                        second_brace_encountered = false;
                        continue;
                    } else {
                        accumulator = String::from("");
                        first_brace_encountered = false;
                        first_number = String::from("");
                        comma_encountered = false;
                        second_number = String::from("");
                        continue;
                    }
                }
            }
        }
    }
    total_product
}
