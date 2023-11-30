// Rust programme for Researcher's Publication Incentive System  Nigerian Researcher's guide
// RPIS is for only 500 researchers

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    loop{

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    println!("How many papers have you published: ");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let papers:u32 = input2.trim().parse().expect("Not a valid number");
    println!("Enter your researcher number: ");
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let researcher_no:u32 = input3.trim().parse().expect("Not a valid number");


    if papers >= 3 && papers <=5 
    {
        println!("Your incentive is N500,000");
    }
    else if papers >= 5 && papers <=10
    {
        println!("Your incentive is N800,000");
    }
    else if papers >=10
    {
        println!("Your incentive is N1,000,000");
    }
    else 
    {
        println!("Your Incetive is N100,000");
    }
    if researcher_no > 500
    {
        break;
    }
}
}