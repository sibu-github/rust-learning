// ** Immutable variables **
// fn main() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// ** Variable shadowing **
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let mut x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//         x += 1;
//         println!("The variable in the inner scope in mutable: {}", x);
//     }

//     println!("The value of x is: {}", x);
// }


fn main() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
}
