// Rust program to finding the roots of a quadratic equation

use std::io;

fn main()
{
    // Creating variables and getting input for values a, b and c
    println!("Input value of a >>");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:i32 = a.trim().parse().expect("Failed to input");

    println!("Input value of b >>");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:i32 = b.trim().parse().expect("Failed to input");

    println!("Input value of c >>");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:i32 = c.trim().parse().expect("Faild to input");

    // Creating a variable for discrimant
    let disc = b * 2 - 4 * a * c;

    // Compare if
    // Discriminant > 0, if yes print command
    if disc > 0
    {
        println!("There are two distinct roots.");
    }

    // Discriminant == 0, if yes print command
    else if disc == 0
    {
        println!("There is one distinct root.");
    }

    // Discriminant < 0, if yes print command.
    else if disc < 0
    {
        println!("There are no real roots.");
    }
}