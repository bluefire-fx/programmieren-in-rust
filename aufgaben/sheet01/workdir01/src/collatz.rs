pub fn calculate_from_start_value(mut n: u64) {
    let mut i:u64 = 0;
    while n != 1 {
        i += 1;
        if n % 2 == 0 {
            n /= 2;
        }
        else {
            n *= 3;
            n += 1;
        }
        println!("{:?} -> {:?}", i, n)
    }
}
