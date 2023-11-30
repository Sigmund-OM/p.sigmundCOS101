// Rust Program for multiplication of 1 to n
// Nested loop.

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    


    println!("What number would you like to stop at? ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let number:i64  = input1.trim().parse().expect("Not valid number");

    println!("What times table would you like? ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let times_table:i64  = input2.trim().parse().expect("Not valid number");

    let mut x = 0;
    println!("The {} times table ",times_table);
    loop{
        x+=1;
        let product = x * times_table;
        println!("x= {} x {}= {}",x,times_table,product);

        if x == number{
            break;
        }
        }  
}
