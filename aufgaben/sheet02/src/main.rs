mod task1;
mod task2;

fn main() {
    main_task1();
}

fn main_task1() {
    for n in 0..21 {
        if task1::prime::is_prime(n) {
            println!("{:?}*", n);
        }
        else {
            println!("{:?}", n);
        }
    }
}