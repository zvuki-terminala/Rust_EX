use std::io;

fn main() {
    println!("Celsium to Farenheit");
    let mut count = 0;

    'Test_1: loop {
        let mut cels = String::new();
        println!("Введите значение по Цельсию");

        io::stdin().read_line(&mut cels).expect("Failed to readline");

        let cels: f32 = match cels.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };
        
        let far: f32 = (cels * 1.8) + 32.0;
        count += 1;
        println!("Farenheit = {far}");
        
        if count == 3 {
            break 'Test_1;
        }
    }
    println!("Fibonacci n...");

    let mut fib_num = String::new();

    println!("Введите n-ое число Фибоначчи");

    io::stdin().read_line(&mut fib_num).expect("Failed to readline");
    let fib_num: u32 = fib_num.trim().parse().expect("Not a number!!!");

    let mut n1: u128 = 0;
    let mut n2: u128 = 1;
    let mut n3: u128;
    let mut counter = 0;
    
    'Test_2: loop {
        if fib_num < 1 {
            break 'Test_2;
        }
        else {
            if counter <= fib_num {
                n3 = n2;
                n2 += n1;
                n1 = n3;
                counter += 1;
            }
            if counter == fib_num {
                break 'Test_2;
            }
        }
        println!("{n2}");
    }
}
