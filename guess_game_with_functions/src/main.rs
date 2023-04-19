//Стандартный ввод вывод
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess number!");

    //let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = random_num();

    loop {
        println!("Plese input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");
        /*
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        */
        let new_guess = str_to_num(guess);

        println!("You guessed: {new_guess}");

        match new_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
/*
    Важно - функция содержит внутри выполняемое выражение,
    если поставить ";" выражение становится оператором, что вызывает ошибку
*/
fn random_num() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn str_to_num(x: String) -> u32 {
    x.trim().parse().expect("Not number")
}