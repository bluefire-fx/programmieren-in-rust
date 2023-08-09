use crate::task1::rule90::next_step;

mod task1;
mod task2;

fn main() {
    task1_main();
}

fn task1_main() {
    let life_cycles = 500000;
    let mut life_line = task1::rule90::read_input();
    for _ in 0..life_cycles {
        for &i in &life_line {
            print!("{}", if i { "██" } else { "  " })
        }
        println!();
        life_line = next_step(&life_line);
    }
}
