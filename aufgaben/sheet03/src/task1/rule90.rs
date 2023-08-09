//! Task 3.1: Rule 90

pub fn main() {
    let life_cycles = 500000;
    let mut life_line = read_input();
    for _ in 0..life_cycles {
        for &i in &life_line {
            print!("{}", if i { "██" } else { "  " })
        }
        println!();
        life_line = next_step(&life_line);
    }
}

/// Reads a valid initial configuration for our automaton from the terminal.
pub fn read_input() -> Vec<bool> {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }

        buffer.trim().to_string()
    };

    // Task 1a)
    let input_as_vec = {
        let mut v = Vec::with_capacity(input.len());
        for c in input.chars() {
            v.push(c == '1')
        }
        v
    };
    input_as_vec
}

// Task 1b)
pub fn next_step(old_step: &[bool]) -> Vec<bool> {
    let mut next_arr: Vec<bool> = Vec::with_capacity(old_step.len());
    for i in 0..old_step.len() {
        next_arr.push(if i == 0 {
            next_cell(&old_step[old_step.len() - 1], &old_step[i + 1])
        } else if i == old_step.len() - 1 {
            next_cell(&old_step[i - 1], &old_step[0])
        } else {
            next_cell(&old_step[i - 1], &old_step[i + 1])
        })
    }
    next_arr
}

fn next_cell(left: &bool, right: &bool) -> bool {
    *left && !*right || !*left && *right
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[true, false, false]), vec![false, true, true]);
    assert_eq!(next_step(&[true, true, false]), vec![true, true, false]);
    assert_eq!(next_step(&[true, true, true]), vec![false, false, false]);
}
