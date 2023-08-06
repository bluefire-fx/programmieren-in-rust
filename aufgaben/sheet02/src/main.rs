mod task1;
mod task2;

fn main() {
    main_task1();
    main_task2();
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

fn main_task2() {
    let mut arr = [61, 21, 27, 79, 57, 60, 46, 42, 27, 92, 66, 26];
    println!("{:?}", arr);
    task2::sort::sort(&mut arr);
    println!("{:?}", arr);

}