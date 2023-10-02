// print if they are both
pub fn main() -> () {
    for iterationnumber in &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20] {
        let iterationnumber = *iterationnumber;
        if !happy_prime(iterationnumber) {} else {
            print!("{}", iterationnumber);
            println!(" is a happy prime!");
        }
    }

}

// is it botH?
fn happy_prime(n: i32) -> bool {
    match check_if_number_is_happy(n) {
        false => return false,
        _ => {}
    }

    if check_if_number_is_prime(n) == true {
        return true;
    } else {
        return false;
    }
}

// Is it a happy number? https://en.wikipedia.org/wiki/Happy_number
fn check_if_number_is_happy(number: i32) -> bool {
    let mut  number: i32= number;

    while number > 1 {
        let mut tmp = 0;
        while number > 0 {
            tmp = tmp + (number %10) * (number%10);
            number = number / 10;
        }
        number = tmp;

        // We ended up in a cycle -> not happy
        if (number == 4) {
            return false;
        }
    }

    return true;
}

// is it prime?
fn check_if_number_is_prime(number: i32) -> bool {
    if number < 3 {
        return false
    }
    for divider in 2..number {
        if number % divider == 0 {
            return false
        }
    }
    return true
}
