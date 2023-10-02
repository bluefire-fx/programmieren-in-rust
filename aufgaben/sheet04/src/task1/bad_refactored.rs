// print if they are both
pub fn main() -> () {
    for number in 1..21 {
        if happy_prime(number) {
            println!("{} is a happy prime!", number);
        }
    }
}

// is it botH?
fn happy_prime(n: u32) -> bool {
    check_if_number_is_happy(n) && check_if_number_is_prime(n)
}

// Is it a happy number? https://en.wikipedia.org/wiki/Happy_number
fn check_if_number_is_happy(mut number: u32) -> bool {
    while number > 1 {
        let mut sum = 0;
        while number > 0 {
            sum += (number %10) * (number%10);
            number /= 10;
        }
        number = sum;

        // We ended up in a cycle -> not happy
        if number == 4 {
            return false;
        }
    }

    return true;
}

// is it prime?
fn check_if_number_is_prime(number: u32) -> bool {
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
