pub fn chars_in_string(s: &str, c: char) -> u32 {
    let mut n_char = 0;
    for i in s.chars() {
        if c == i {
            n_char += 1;
        }
    }
    n_char
}
