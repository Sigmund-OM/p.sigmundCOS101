//Program to calculate how fast a car is travelling

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the distance covered by the car(in miles):");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let d:f32 = input1.trim().parse().expect("Not a valid number");
    
    //change to kilometers d = d*1.609344;
    println!("The distance in kilometers is: {}",d);

    println!("Enter the time taken to cover this distance:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f32 = input2.trim().parse().expect("Not a valid number");

    //Finding speed 
    let unit = "Kilometers/Hour";
    let speed = d / t;
    println!("The speed of the car is: {} {}",speed,unit);

}
