fn main() {
    // Pass temperature and to what it should be converted in format 'C' for converting celsius and 'F' for farenheit covnersion
    convert(100.0, 'C');
    convert(38.6, 'F');
    convert(38.6, 'Z');
}

fn convert(temp: f32, to: char) {
    if to == 'C' {
        let result = (temp - 32.0) / 1.8;
        println!("{temp} F is {result} C");
    } else if to == 'F' {
        let result = (temp * 1.8) + 32.0;
        println!("{temp} C is {result} F");
    } else {
        println!("Invalid input, only 'C' or 'F' are accepted for conversion");
    }
}
