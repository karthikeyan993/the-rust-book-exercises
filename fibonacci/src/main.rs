use std::io;

fn main() {
    'fibo_loop: loop {
        println!(
            "Please enter which Fibonacci number you'd like to calculate (e.g., 1st, 10th, etc.):"
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        let number: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter number between 1 - 100");
                continue;
            }
        };
        if number > 100 {
            println!("Please enter number between 1 - 100");
            continue 'fibo_loop;
        }
        fibonacci(number);
        loop {
            println!("c - continue, q - quit");
            let mut input2 = String::new();
            io::stdin()
                .read_line(&mut input2)
                .expect("Failed to read the line");

            let exit: char = match input2.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if exit == 'q' {
                break 'fibo_loop;
            } else if exit == 'c' {
                continue 'fibo_loop;
            }
        }
    }
}

fn fibonacci(n: u32) {
    // Assuming fibonacci 1st entry starts from 1
    if n <= 2 {
        if n == 1 {
            println!("the no 1 number in fibonacci series is 0");
        } else if n == 2 {
            println!("the no 2 entry in fibonacci series is 1");
        } else {
            println!("Invalid input")
        }
    } else {
        let mut index = 3;
        let mut first: u128 = 0;
        let mut second: u128 = 1;
        while index <= n {
            // Adding first and second and assigning the second to first and sum to second number
            let mut _sum: u128 = 0;
            _sum = first + second;
            first = second;
            second = _sum;
            index += 1;
        }
        println!("the no {n} entry in fibonacci series is {second}");
    }
}
