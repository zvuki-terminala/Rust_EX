//Стандартный ввод вывод
use std::io;

//Добавление функции сравнения в use блок и указание трейта cmp (компаратор) позволяет использовать Ordering без std::cmp
//Похоже на неймспейсы в C++
use std::cmp::Ordering;

//Для использования функций используется формат: Ящик::Черта, Черта (Трейт) определяет используемые методы
//В частности: Rng трейт отккрывает доступ к функции gen_range в крейте rand
use rand::Rng;

//Функция майн
fn main() {
    println!("Guess number!");

    //Обьявление немутабельной переменной, вызов функций рандома: Крейт::Функция().Черта()
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    //Непараметризованный цикл
    /*
        Что интересно, наличие исключения без повторного цикла ввода переменной не спасает
        от выхода из цикла, из-за ошибки вводимого значения с типом
    */
    loop {
        println!("Plese input your guess.");

        //Определиние изменяемой переменной строкового типа
        let mut guess = String::new();

        //Вызов функции чтения с клавиатуры
        io::stdin()
            .read_line(&mut guess) //Запись введенной строки в изменяемую ссылку
            .expect("Failed to readline"); //Исключение

        /*
            Затенение ранее определенной переменной guess, по сути преобразование типа.
            Важно, для сохранения значения использовать преобразование надо
            после определения предыдущего типа переменной

            Само преобразование: let <Переменная>: <Новый тип> = <Переменная>.<Удаление пробелов>.<Преобразование числа в строку>.<Исключение>
            let guess: u32 = guess.trim().parse().expect("Please type a number!");
        */ 
        //Обработка исключения и обьединение обьявления переменной с обработкой match'ем
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); //Вывод с шаблоном

        /*
            match сопоставляет значения с шаблонами, использует хэндлеры в виде перечисления ветвей, можно использовать как оператор выхода из цикла
            Т.к. match находится в конце цикла, его прерывание приведет к выходу из цикла

            Шаблон:
            match <Выражение> {
                Хэндлер1 => Вывод1,
                Хэндлер2 => Вывод2,
                ...
                ХэндлерI => {
                    ВыводI;
                    break;
                }
            }

            Аналогичен case в C/C++
        */

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

//У разных крейтов разные методы использования, для получения документации используется cargo doc --open
//После ряда обновлений программа получила следующий цикл работы:
/*
    Обьявляем числовую переменную secret_number
    Определяем ее
    Вход в цикл
    Обьявляем строку guess
    Определяем ее
    Преобразуем тип guess в числовой
    Используем оператор ветвления

    Выход из цикла в случае положительного результата ветви с функцией break
    В противном случае перезапуск цикла
*/