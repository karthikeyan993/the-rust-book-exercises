use std::io;
fn main() {
    // Pass temperature and to what it should be converted in format 'C' for converting celsius and 'F' for farenheit covnersion
    // convert(100.0, 'C');
    // convert(38.6, 'F');
    // convert(38.6, 'Z');
    'temp_loop: loop {
        println!("Please enter the temperature value: ");
        let mut input1 = String::new();
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");

        let temp: f32 = match input1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please '1' for C->F, '2' for F->C");

        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read the line");
        let to: u8 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        convert(temp, to);
        loop {
            let mut input3 = String::new();

            println!("c - continue, q - quit");

            io::stdin()
                .read_line(&mut input3)
                .expect("Failed to read line");

            let exit: char = match input3.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if exit == 'q' {
                break 'temp_loop;
            } else if exit == 'c' {
                continue 'temp_loop;
            }
        }
    }
}

fn convert(temp: f32, to: u8) {
    if to == 2 {
        let result = (temp - 32.0) / 1.8;
        println!("{temp} F is {result} C");
    } else if to == 1 {
        let result = (temp * 1.8) + 32.0;
        println!("{temp} C is {result} F");
    } else {
        println!("Invalid input, only 'C' or 'F' are accepted for conversion");
    }
}
