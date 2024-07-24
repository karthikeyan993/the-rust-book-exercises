fn main() {
    fibonacci(1);
    fibonacci(2);
    fibonacci(0);
    fibonacci(4);
    fibonacci(11);
}

fn fibonacci(n: u32) {
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
        let mut first = 0;
        let mut second = 1;
        while index <= n {
            let mut _sum = 0;
            _sum = first + second;
            first = second;
            second = _sum;
            index += 1;
        }
        println!("the no {n} entry in fibonacci series is {second}");
    }
}
