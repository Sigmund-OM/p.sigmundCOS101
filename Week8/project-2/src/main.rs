// Programme for Ernest & Young Global limited 

fn main() {
   // Create empty vectors 
   let mut interview_num : Vec<i64> = Vec::new();
   let mut name : Vec<String> = Vec::new();
   let mut age  : Vec<i32> = Vec::new();
   let mut marital_status : Vec<bool> = Vec::new();
   let mut work_experience : Vec<i32> = Vec::new();
  
   // Push new elements into
   let mut input1 = String::new();
   println!("How many Interviewees are we having today?");
   std::io::stdin().read_line(&mut input1).expect("Not a valid string");
   let interviewee_num:i32 = input1.trim().parse().expect("Not a valid integer");
  
   let mut x = 0;
  for count in 0..interviewee_num {
    let mut input2 = String::new();
    println!("Enter interview number {}", count+1);
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let new_interview_num:i64 = input2.trim().parse().expect("Invalid Input");
    interview_num.push(new_interview_num);

    let mut input3 = String::new();
    println!("Enter your name: ");
    std::io::stdin().read_line(&mut input3).expect("Not a valid string");
    let new_name:String = input3.trim().parse().expect("Invalid Input");
    name.push(new_name);

    let mut input4 = String::new();
    println!("Enter your age: " );
    std::io::stdin().read_line(&mut input4).expect("Not a valid string");
    let new_age:i32 = input4.trim().parse().expect("Not a valid integer");
    age.push(new_age);

    let mut input5 = String::new();
    println!("Are you married?" );
    std::io::stdin().read_line(&mut input5).expect("Not a valid string");
    let new_marital_status:bool = input5.trim().parse().expect("Invalid Input");
    marital_status.push(new_marital_status);

    let mut input6 = String::new();
    println!("How many years of work experience do you have?" );
    std::io::stdin().read_line(&mut input6).expect("Not a valid string");
    let new_work_experience:i32 = input6.trim().parse().expect("Invalid Input");
    work_experience.push(new_work_experience);  

    
    if work_experience > x
    {
    let x = work_experience;
    let y = work_experience[i];
    }
   }

   // Determining the interviewee with the most work experience

   
   println!("The employee with the highest work experience:
             Name: {}
             Age: {}
             Marital Status: {}
             Work experience: {}",name[i],age[i],marital_status[i],y);  
   
}