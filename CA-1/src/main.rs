// Rust program to determine certain discounts given to patients 
// and then print if the person is given discount
// or not given discount

use std::io;

fn main() {}

    // input name
    println!("\n Please enter your name");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name is {}", name);

    // input date of birth
    println!("\n Please enter your date of birth");
    let mut dob = String::new();
        io::stdin()
        .read_line(&mut dob)
        .expect("Failed to read input");
    println!("Your date of birth is {}",dob);
    
    // input email address
    println!("\n Please enter your email address ");
    let mut email = String::new();
        io::stdin()
        .read_line(&mut email) 
        .expect("Failed to read input");
    println!("Your email address is {}",email);  

    // input phone number
    println!("\n Please enter your phone number");
    let mut phone = String::new();
        io::stdin()
        .read_line(&mut phone)
        .expect("Failed to read input");
    println!("Your phone numberis {}",phone);

    //input number of siblings 
    println!("\n Please enter the number of your siblings");
    let mut siblings = String::new();
        io::stdin()
        .read_line(&mut siblings) 
        .expect("Failed to read input"); 
    println!("In total your siblings are {} ",siblings ); 

    // input number of children 
    println!("\n Please enter the number of your children");
    let mut children = String::new();
        io::stdin()
        .read_line(&mut children) 
        .expect("Failed to read input"); 
    println!("In total your children are {} ",children ); 

    // input medical diagnosis
    println!("\n Please enter your medical diagnosis");
    let mut diagnosis = String::new();
        io::stdin()
        .read_line(&mut diagnosis) 
        .expect("Failed to read input"); 
    println!("You are diagnosed of {} ",diagnosis ); 

    // input village
    println!("\n Please enter the name of your village");
    let mut village = String::new();
        io::stdin()
        .read_line(&mut village) 
        .expect("Failed to read input"); 
    println!("The name of your village is {} ",village ); 
    
    
    let mut input = String::new();

    println!("\n Please enter your age");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
        let age:i32 = age.trim().parse().expect("Input not an integer");

    if age > 50 
    if village = Akpabom 
    if children > 4 
    if diagnosis = Alzheimer
    {
        println!("You are given a 20% discount");
    }

    if age = 30
    if village = Ngbauji
    if siblings > 4 
    if diagnosis = Arrhythmia
    {
        println!("You are given a 5% discount");
    }

    if age > 40
    if village = Atabrikang
    if children > 3
    if siblings > 3
    if diagnosis = Chronic Kidney Disease
    {
        println!("You are given a 15% discount");
    }

    if age > 58 
    if village = Emeremen 
    if children > 5
    if siblings > 5
    if diagnosis = Arthritis
    {
        println!("You are given a 10% discount");
    }


} 

      
