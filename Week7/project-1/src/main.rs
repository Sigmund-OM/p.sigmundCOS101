use std::io;

fn trapezium_area_calculator(){
    let mut input1 = String::new();
    println!("Enter the height of the trapezium: ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let h:f32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
    println!("Enter the first base of the trapezium: ");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let b1:f32 = input2.trim().parse().expect("Not a valid number");

    let mut input3= String::new();
    println!("Enter the second base  of the trapezium: ");
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let b2:f32 = input3.trim().parse().expect("Not a valid number");

    let area = (h/2.0)*(b1+b2);
    println!("The area of the trapezium is:{} ",area);
}
fn rhombus_area_calculator(){
    let mut input1 = String::new();
    println!("Enter the first diagonal of the rhombus: ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let d1:f32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
    println!("Enter the second diagonal of the rhombus: ");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let d2:f32 = input2.trim().parse().expect("Not a valid number");
 
    let area = (1.0/2.0)*d1*d2;
    println!("The area of the rhombus is: {}",area);
}
fn parallelogram_area_calculator(){
    let mut input1 = String::new();
    println!("Enter the base of the parallelogram: ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let b:f32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
    println!("Enter the altitude of the parallelogram: ");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let a:f32 = input2.trim().parse().expect("Not a valid number");
 
    let area = b*a;
    println!("The area of the parallelogram is: {}",area);
}
fn cube_area_calculator(){
    let mut input1 = String::new();
    println!("Enter a side of the cube: ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let s:f32 = input1.trim().parse().expect("Not a valid number");
  
    let area =6.0*s.powf(2.0);
    println!("The area of the cube is: {}",area);
}
fn cylinder_area_calculator(){
    let mut input1 = String::new();
    println!("Enter the radius of the cylinder: ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let r:f32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
    println!("Enter the height of the cylinder: ");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let h:f32 = input2.trim().parse().expect("Not a valid number");
 
    let area = (22.0/7.0)*r.powf(2.0)*h;
    println!("The area of the cylinder is: {}",area);
}
fn main() {
    println!("Welcome to the shape calculator.
              You are to choose from the following shapes:
              1- Area of a trapezium
              2- Area of a rhombus
              3- Area of a parallelogram
              4- Area of a cube 
              5- Area of a cylinder");

    let mut input1 = String::new();
    println!("Which one would you like to use?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let shape:i32 = input1.trim().parse().expect("Not a valid number");

    if shape == 1
    {
      trapezium_area_calculator();
    }
    else if shape == 2
    {
        rhombus_area_calculator();
    }
    else if shape == 3
    {
        parallelogram_area_calculator();
    }
    else if shape == 4
    {
        cube_area_calculator();
    }
    else if shape == 5
    {
        cylinder_area_calculator();
    }
    else 
    {
        println!("Sorry, you cannot calculate for this shape.");
    } 
}