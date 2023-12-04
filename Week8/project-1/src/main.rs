fn main() {
    let mut interview_num: Vec<i64> = Vec::new();
    let mut name: Vec<String> = Vec::new();
    let mut age: Vec<i32> = Vec::new();
    let mut marriage_status: Vec<bool> = Vec::new();
    let mut work_exp: Vec<i32> = Vec::new();

    let mut input1 = String::new();
    println!("How many Interviewees are we having today?");
    std::io::stdin().read_line(&mut input1).expect("Not a valid string");
    let interviewee_num: i32 = input1.trim().parse().expect("Not a valid integer");

    let mut max_ind = 0; // Track the index of the interviewee with the highest work experience

    for count in 0..interviewee_num {
        let mut input2 = String::new();
        println!("Enter interviewee {}'s interview number:", count + 1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_interview_num: i64 = input2.trim().parse().expect("Invalid Input");
        interview_num.push(new_interview_num);

        let mut input3 = String::new();
        println!("Enter interviewee {}'s name:", count + 1);
        std::io::stdin().read_line(&mut input3).expect("Not a valid string");
        let new_name: String = input3.trim().parse().expect("Invalid Input");
        name.push(new_name);

        let mut input4 = String::new();
        println!("Enter interviewee {}'s age:", count + 1);
        std::io::stdin().read_line(&mut input4).expect("Not a valid string");
        let new_age: i32 = input4.trim().parse().expect("Not a valid integer");
        age.push(new_age);

        let mut input5 = String::new();
        println!("Is interviewee {} married? (true/false):", count + 1);
        std::io::stdin().read_line(&mut input5).expect("Not a valid string");
        let marriage_status_input: String = input5.trim().parse().expect("Invalid Input");
        let marriage_status_bool: bool = marriage_status_input.parse().expect("Invalid Input");
        marriage_status.push(marriage_status_bool);

        let mut input6 = String::new();
        println!("Enter interviewee {}'s years of work experience:", count + 1);
        std::io::stdin().read_line(&mut input6).expect("Not a valid string");
        let new_work_exp: i32 = input6.trim().parse().expect("Invalid Input");
        work_exp.push(new_work_exp);

        // Update the maximum work experience and its index
        if new_work_exp > work_exp[max_ind] {
            max_ind = count as usize;
        }
    }

    // Access the information of the employee with the highest work experience
    println!("The interviewee with the highest work experience:\nName: {}\nAge: {}\nMarital Status: {}\nWork experience: {}",
        name[max_ind], age[max_ind], marriage_status[max_ind], work_exp[max_ind]);
}