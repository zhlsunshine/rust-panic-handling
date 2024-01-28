use std::io;
use std::io::Write;
use std::panic;

fn main() {
    let mut try_times: i32 = 0;
    let mut int_array: [i32; 3] = [0; 3];
    println!("\n ###### Divide by zero ###### \n");
    while try_times < 3 {
        let current_time = try_times as usize;

        // Handle divide by zero panic
        let result_value = panic::catch_unwind(|| {
            println!("This is the {}th to handle panic.", current_time);
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
            numerator / denominator
        });

        match result_value {
            Ok(result) => {
                println!("No panic occur and the result is: {:?}", result);
                int_array[current_time] = result;
            },
            Err(e) => {
                if let Some(err) = e.downcast_ref::<&str>() {
                    println!("Caught panic: {}", err);
                } else {
                    println!("Caught panic of unknown type");
                }
            },
        };

        try_times += 1;
        println!("##########################################");
    }

    println!("\n @@@@@@ Iteration @@@@@@ \n");

    for i in 0..=3 {
        // Handle out of index range panic
        let num_result = panic::catch_unwind(|| {
            println!("Iterate Element: {}", int_array[i]);
        });

        match num_result {
            Ok(()) => {
                println!("No panic occur for this iteration");
            },
            Err(e) => {
                if let Some(err) = e.downcast_ref::<&str>() {
                    println!("Caught panic: {}", err);
                } else {
                    println!("Caught panic of unknown type");
                }
            },
        };
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    }
    
    println!("Complete the panic handle examples!");
}

