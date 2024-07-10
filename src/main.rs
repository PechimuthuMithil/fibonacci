use std::io;

fn fibonacci(n: i32) -> i64 {
    if n < 1 {
        -1
    } else if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    loop {
        let mut n = String::new();
        println!("Enter N (type 'exit' to Exit): ");

        io::stdin().read_line(&mut n).expect("Failed to read the value of N");
        
        let n = n.trim();
        
        if n.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }
        
        let n: i32 = match n.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value for N, exiting...");
                break;
            }
        };
        
        let result = fibonacci(n);
        
        if result == -1 {
            println!("Expected N >= 1, got {}", n);
        } else {
            println!("The {}th Fibonacci number is: {}", n, result);
        }
    }
}
