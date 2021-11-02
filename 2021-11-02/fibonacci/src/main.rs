use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter a number.");
    io::stdin().read_line(&mut input).unwrap();
    let num:i32 = input.trim().parse().unwrap();
    println!("The {}th fibonacci number is {}", num, get_fibo_num(num));
}


fn get_fibo_num( n :i32 ) -> i32 {
    let mut n_minus_2 = 0;
    let mut n_minus_1 = 1;
    let mut curr = 1;
    for _ in 1..n {
        curr = n_minus_1 + n_minus_2;
        n_minus_2 = n_minus_1;
        n_minus_1 = curr;
    }
    curr
}


