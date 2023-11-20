// Rust prgram to determine the order of customers
// and then print the person's bill if given with discount
// or not given

use std::io;

fn main() {

    let mut name = String::new();
    let mut price:i32 = 0;
    
    
    
    // input name
    println!("\n Please enter your name");
    io::stdin().read_line(&mut name).expect("not a valid string");       
    println!("Your name is {}", name);

    // input food 
    println!("Enter the letter of the food would you like to order (q to quit)");
    

    
loop{
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("not a valid string");

    let input1 = input1.trim();
    if input1 == "P" 
    {
         price+= 3_200;    
    }
    else if input1 == "F"
    {
         price += 3_000;
    }
    else if input1 == "A"
    {
        price +=2_500;
    }
    else if input1 == "E"
    {
        price +=2_000;
    }
    else if input1 == "W"
    {
        price +=2_500;
    }
    else if input1 == "q"
    {
        break;
    }
    else
    {
        println!("We dont sell that here.");
        continue;
    }
    }
    println!("Your total price is {}",price);
    if price > 10_000
    {
    let  new_price =  (price - (5/100 * price));
    
    println!("Because your total is greater tha 10,000 your new price is {}",new_price);
   }
    else {
        {
            println!("your balance remains the same");
        }
    }
    println!("Thank you and have a good day.");


}
