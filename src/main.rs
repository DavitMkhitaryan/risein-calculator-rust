use std::io;

fn main() {
    println!("Please enter the first number: ");
    let mut first_number_input = String::new();
    io::stdin()
        .read_line(&mut first_number_input)
        .expect("Failed to read line");
    let trimmed_first_number = first_number_input.trim();

    let first_number = match trimmed_first_number.parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("That's not a valid number");
            return;
        }
    };

    println!("Please enter the operation name: ");
    let mut operation_input = String::new();
    io::stdin()
        .read_line(&mut operation_input)
        .expect("Failed to read line");
    let operation_input = operation_input.trim();

    println!("Please enter the second number: ");
    let mut second_number_input = String::new();
    io::stdin()
        .read_line(&mut second_number_input)
        .expect("Failed to read line");
    let trimmed_second_number = second_number_input.trim();
    let second_number = match trimmed_second_number.parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("That's not a valid number");
            return;
        }
    };
    
    let result: Result<f64, String>;

    match operation_input {
        "Add" => {
            result = calculate(Operation::Add { x: first_number, y: second_number })
        },
        "Subtract" => {
            result = calculate(Operation::Subtract { x: first_number, y: second_number })
        },
        "Multiply" => {
            result = calculate(Operation::Multiply { x: first_number, y: second_number })
        }
        "Divide" => {
            result = calculate(Operation::Divide { x: first_number, y: second_number })
        }
        _ => {
            println!("Invalid operation");
            return
        }
    }

    match result {
        Ok(num) => {
            println!("Result is {}", num)
        },
        Err(error) => {
            print!("{}", error)
        }
    }

}

enum Operation {
    Add { x: f64, y: f64 },
    Subtract { x: f64, y: f64 },
    Multiply { x: f64, y: f64 },
    Divide { x: f64, y: f64 }
}

fn calculate(operation: Operation) -> Result<f64, String> {
    match operation {
        Operation::Add { x, y } => {
            return Ok(x+y);
        }
        Operation::Subtract { x, y } => {
            return Ok(x-y);
        }
        Operation::Multiply { x, y } => {
            return Ok(x*y);
        }
        Operation::Divide { x, y } => {
            if y == 0.0 {
                return Err(String::from("Cannot divide by 0"));
            }
            return Ok(x/y);
        }
    }
}