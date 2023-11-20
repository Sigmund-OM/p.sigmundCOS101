// Rust program to determine certain incentives given to employees
// and then print if the person is given the incentive
// or not given



use std::io;

fn main() {

      let mut name = String::new();
      let mut input1 = String::new();
      let mut input2 = String::new();
    

    // input name
    println!("\n Please enter your name");
    io::stdin().read_line(&mut name).expect("not a valid string");       
    println!("Your name is {}", name);
   
   // input experience
    println!("\n Are you experienced? ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experience:bool = input1.trim().parse().expect("Not valid");
 
   // input age
   println!("Enter your age: ");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let age:i32 = input2.trim().parse().expect("Not a valid number");

   if experience == true && age >= 40
   {
     println!(" Your incentive is N1,560,000 per month");
   }
   else if experience == true && age >= 30 && age < 40 
   {
     println!(" Your incentive is N1,480,000 per month");
   }
   else if experience == true && age < 28
   {
     println!("Your incentive is N1,300,000 per month");
   }
   else if experience == false
   {
     println!("Yout incentive is N100,000");
   }
   else
   {
    println!("Sorry We cannot employ you here");
   }

}