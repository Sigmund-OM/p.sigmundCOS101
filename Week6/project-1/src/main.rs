// Rust program for Student Council Voting System
// Program only works fir the first 150 ppl.

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();


loop{
println!("Enter your name: ");
io::stdin().read_line(&mut input1).expect("Not a valid String");
println!("Enter your E-mail Address: ");
io::stdin().read_line(&mut input2).expect("Not a valid String");
println!("Enter your Department: ");
io::stdin().read_line(&mut input3).expect("Not a valid String");
println!("Enter your State of origin: ");
io::stdin().read_line(&mut input4).expect("Not a valid String");
println!("Are you a Class Representatative?");
io::stdin().read_line(&mut input5).expect("Not a valid String");
let C_rep:bool = input5.trim().parse().expect("Not valid");
println!("What level are you?");
io::stdin().read_line(&mut input6).expect("Not a valid String");
let level:u32 = input6.trim().parse().expect("Not a valid number");
println!("What is your CGPA?");
io::stdin().read_line(&mut input7).expect("Not a valid String");
let cgpa:f32 = input7.trim().parse().expect("Not a valid number");
println!("Enter your candidate number: ");
io::stdin().read_line(&mut input8).expect("Not a valid String");
let candidate_no:u32 = input8.trim().parse().expect("Not a valid number");

if C_rep == false && cgpa > 4.0 && level == 200 || level == 300 || level == 400
{
    println!("Your name is: {}
          Your E-mail is: {} 
          Your Department is: {}
          Your State of Origin is: {}
          You can vote.",input1, input2, input3, input4);  
}   
else
{
   println!("Sorry, You are not eligible to vote.");
}
if candidate_no > 150
{
    break;
}
}
}