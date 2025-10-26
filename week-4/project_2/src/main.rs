use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Are you experienced?, true/false:");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let experience:bool = input1.trim().parse().expect("You can only input yes or no");

    println!("Enter your age:");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if experience == true && age >= 40 {
        println!("incentive of employee is 1_560_000");
    }

    else if experience == true && age >= 30 && age < 40 {
        println!("incentive of employee is 1_480_000");
    }

    else if experience == true && age < 28 {
        println!("incentive of employee is 1_300_000");
    }

    else if experience == false {
        println!("incentive of employee is 100_000");
    }
    

}
