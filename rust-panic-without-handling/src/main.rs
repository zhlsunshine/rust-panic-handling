use std::io;
use std::io::Write;

fn main() {
    let mut try_times: i32 = 0;
    let mut int_array: [i32; 3] = [0; 3];
    println!("\n ###### Divide by zero ###### \n");
    while try_times < 3 {
        let current_time = try_times as usize;
        
        // Get numerator from user input
        let mut numerator = String::new();
        print!("Please input the numerator: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut numerator).expect("Failed to read line");
        let numerator: i32 = numerator.trim().parse().expect("Invalid input");
        
        // Get denominator from user input
        let mut denominator = String::new();
        print!("Please input the denominator: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut denominator).expect("Failed to read line");
        let denominator: i32 = denominator.trim().parse().expect("Invalid input");
        
        // Perform division without validation
        int_array[current_time] = numerator / denominator;
        println!("Result is: {:?}", int_array[current_time]);
        try_times += 1;
        println!("##########################################");
    }

    println!("\n @@@@@@ Iteration @@@@@@ \n");
    for i in 0..=3 {
        println!("Iterate Element: {}", int_array[i]);
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    }
    
    println!("Complete the panic handle examples!");
}
